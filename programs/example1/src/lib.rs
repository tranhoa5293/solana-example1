use anchor_lang::prelude::*;

declare_id!("7MvdKbxE1XiuayNVFZFsmGXn2Z4Afb38XQuMC2hbVAMc");

#[program]
pub mod example1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        // let my_account = &mut ctx.accounts.my_account;
        // my_account.data = data;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 80)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}


#[account]
pub struct MyAccount {
    pub data: u64
}