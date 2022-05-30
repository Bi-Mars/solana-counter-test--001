use anchor_lang::prelude::*;

declare_id!("BRBhbua18ePSpvp97RKKZG9sX4jjGS9hwYR6eZ6v7LX8");

#[program]
pub mod test001_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = 0;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data += 1;
        Ok(())
    }

    // This function might crash the code if the number goes below 0 (because it is u64)
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
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

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}

