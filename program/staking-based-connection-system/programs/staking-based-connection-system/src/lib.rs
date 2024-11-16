use anchor_lang::prelude::*;

declare_id!("DanBEhYAqZSphxBtUpbY4ndKkhV5PARTTGNf3qxyYy9B");

#[program]
pub mod staking_based_connection_system {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, name: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.user_pubkey = ctx.accounts.signer.key();
        user.name = name;
        user.req_count = 0;
        user.req_sent = 0;
        Ok(())
    }

    pub fn update_profile_req_count(ctx: Context<UpdateProfile>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.req_count += 1; 
        Ok(())

    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", signer.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(
        mut,
        seeds = [b"user", signer.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user_pubkey: Pubkey,
    #[max_len(20)]
    pub name: String,
    pub req_count: u32,
    pub req_sent: u32,
}
