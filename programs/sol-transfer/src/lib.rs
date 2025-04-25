use anchor_lang::prelude::*;


// pubkey: 2mY1nKy32jNAGcgviqM8hZvmFmrBxUXxSq85uhsXEpxZ
// =============================================================================
// Save this seed phrase and your BIP39 passphrase to recover your new keypair:
// exchange pizza rotate gaze chicken grab glance woman broken also retire crash


declare_id!("2GP1714MWmjJRnv49DyQnjRKwfCVFANmetaEHaprJE1Y");

#[program]
pub mod sol_transfer {
    use super::*;

    pub fn my_command(ctx: Context<CommandAccounts>, plate_number: u64, username: String) -> Result<()> {
        ctx.accounts.data_account.number1 = plate_number;
        ctx.accounts.data_account.number2 = 2;
        // ctx.accounts.data_account.name = "Mercy".to_string();
        ctx.accounts.data_account.name = username;
        msg!("Data account created !");
        Ok(())
    }

    pub fn my_multiplication(ctx: Context<MultiplyAccounts>) -> Result<()> {
        let multiplication: u64 = ctx.accounts.data_account_1.number1 * ctx.accounts.data_account_2.number2;
        msg!("The multiplication is {multiplication} !");
        Ok(())
    }

    pub fn reveal_name(ctx: Context<NameAccounts>) -> Result<()> {
        let nick: String = "Aphomer".to_string();
        let first: String = "Mercy".to_string();
        let last: String = "Afolabi".to_string();

        msg!("My nickname is {}, my Mummy calls me {}, but my courmates calls me {}", nick, first, last);
        Ok(())
    }
}





#[derive(Accounts)]
pub struct CommandAccounts<'info> {
    // #[account(mut)]
    #[account(init, payer = user, space = 8 + 8 +2)]
    pub data_account: Account<'info, AccountStruct>, // we can now use     pub data_account: Account<'info, AccountStruct>, // we can now use     pub data_account: Account<'info, AccountStruct>, // we can now use data_account.number1 // data_account.number2
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

// MULTIPLICATION
#[derive(Accounts)]
pub struct MultiplyAccounts<'info> {
    pub data_account_1: Account<'info, AccountStruct>, // we can now use     pub data_account: Account<'info, AccountStruct>, // we can now use     pub data_account: Account<'info, AccountStruct>, // we can now use data_account.number1 // data_account.number2
    pub data_account_2: Account<'info, AccountStruct>,
}


// FOR NAMES
#[derive(Accounts)]
pub struct NameAccounts<'info> {
    pub nick_name: Account<'info, NameStruct>, // we can now use     pub data_account: Account<'info, AccountStruct>, // we can now use     pub data_account: Account<'info, AccountStruct>, // we can now use data_account.number1 // data_account.number2
    pub first_name: Account<'info, NameStruct>,
    pub last_name: Account<'info, NameStruct>,
}


#[account]
pub struct AccountStruct {
    number1: u64, // we can now use data_account.number1 || data_account.number2
    number2: u64,   // || data_account.number2
    name: String
}

#[account]
pub struct DifferentAccountStruct {
    number1: u64, // we can now use data_account.number1 || data_account.number2
    // number2: u64,   // || data_account.number2
    number2: u16
}


// FOR NAMES
#[account]
pub struct NameStruct {
    nickName: String, // we can now use data_account.number1 || data_account.number2
    firstName: String,   // || data_account.number2
    lastName: String
}






