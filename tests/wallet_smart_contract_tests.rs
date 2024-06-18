use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use mass_payouts::{self, MassPayoutRequest};

#[tokio::test]
async fn test_mass_payouts() {
    // Configure the client to use the local cluster.
    let provider = anchor_lang::Provider::env();
    anchor_lang::Program::set_provider(provider.clone());

    // Initialize the program.
    let program = anchor_lang::Program::new(mass_payouts::ID, provider);

    // Create a new mint and admin token amount.
    let mint = Mint::new(&program.provider().unwrap());
    let admin_token_account = TokenAccount::new(&mint, &program.provider().unwrap().wallet.pubkey());

    // Mint some tokens to the admin token account.
    let mint_amount = 1000;
    token::mint_to(
        &program.provider().unwrap(),
        &mint.pubkey(),
        &admin_token_account.pubkey(),
        &program.provider().unwrap().wallet.pubkey(),
        &[],
        mint_amount,
    )
    .await
    .unwrap();

    // Create a vendor token account.
    let vendor_token_account = TokenAccount::new(&mint, &Pubkey::new_unique());

    // Define the mass payout requests.
    let payouts = vec![
        MassPayoutRequest { amount: 100 },
        MassPayoutRequest { amount: 200 },
        MassPayoutRequest { amount: 300 },
    ];

    // Invoke the mass_payouts instruction.
    let result = program
        .request()
        .accounts(mass_payouts::accounts::ProcessMassPayouts {
            admin: program.provider().unwrap().wallet.pubkey(),
            admin_token_account: admin_token_account.pubkey(),
            vendor_token_account: vendor_token_account.pubkey(),
            token_mint: mint.pubkey(),
            token_program: token::ID,
        })
        .args(mass_payouts::instruction::ProcessMassPayouts { payouts })
        .send()
        .await;

    // Assert that the transaction was successful.
    assert!(result.is_ok());

    // Check the balances of the admin and vendor token accounts.
    let admin_balance = token::accessor::amount(&admin_token_account.pubkey()).await.unwrap();
    let vendor_balance = token::accessor::amount(&vendor_token_account.pubkey()).await.unwrap();

    assert_eq!(admin_balance, mint_amount - 600);
    assert_eq!(vendor_balance, 600);
}
