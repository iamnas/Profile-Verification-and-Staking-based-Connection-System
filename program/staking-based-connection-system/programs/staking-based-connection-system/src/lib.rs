use anchor_lang::prelude::*;

declare_id!("8keiCsLLH96Mcoyr6duSjASAcUyHA7YRHcBMioaTwGPV");

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

    // pub fn update_profile_req_count(ctx: Context<UpdateProfile>) -> Result<()> {
    //     let user = &mut ctx.accounts.user;
    //     user.req_count += 1;
    //     Ok(())
    // }

    pub fn request_connection(ctx: Context<RequestConnection>) -> Result<()> {
        let matched = &mut ctx.accounts.matched;

        matched.from = ctx.accounts.signer.key();
        matched.to = ctx.accounts.to.key();
        matched.is_matched = false;
        // matched.bump = *ctx.bumps.get("matched").unwrap();

        // let user = &mut ctx.accounts.user;
        // user.req_sent += 1;

        // let to_user = &mut ctx.accounts.to_user;
        // to_user.req_count += 1;

        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.signer.key(),
            &ctx.accounts.matched.to_account_info().key(),
            200_000_000, // 0.2 SOL
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.matched.to_account_info(),
            ],
        );
        // if result.is_err() {
        //     return Err(ErrorCode::TransferFailed.into());
        // }

        Ok(())
    }

    pub fn accept_connection(ctx: Context<AcceptConnection>) -> Result<()> {
        let matched = &mut ctx.accounts.matched;

        require!(
            matched.to == ctx.accounts.signer.key(),
            ErrorCode::Unauthorized
        );

        matched.is_matched = true;

        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.signer.key(),
            &ctx.accounts.matched.to_account_info().key(),
            200_000_000, // 0.2 SOL
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.matched.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn reject_connection(ctx: Context<RejectConnection>) -> Result<()> {
        let matched = &ctx.accounts.matched;

        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            &matched.to_account_info().key(),
            &ctx.accounts.from.key(),
            200_000_000, // Refund 0.2 SOL to `from`
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                matched.to_account_info(),
                ctx.accounts.from.to_account_info(),
            ],
        )?;

        // **ctx.accounts.from.to_account_info().lamports.borrow_mut() +=
        //     matched.to_account_info().lamports();
        // **matched.to_account_info().lamports.borrow_mut() = 0;

        Ok(())
    }

    // pub fn withdraw_stake(ctx: Context<WithdrawStake>) -> Result<()> {
    //     let matched = &ctx.accounts.matched;

    //     if matched.is_matched {
    //         // Refund 0.2 SOL to both users
    //         let transfer_to_from = anchor_lang::solana_program::system_instruction::transfer(
    //             &ctx.accounts.matched.to_account_info().key(),
    //             &ctx.accounts.from.key(),
    //             200_000_000,
    //         );
    //         let transfer_to_to = anchor_lang::solana_program::system_instruction::transfer(
    //             &ctx.accounts.matched.to_account_info().key(),
    //             &ctx.accounts.to.key(),
    //             200_000_000,
    //         );

    //         anchor_lang::solana_program::program::invoke(
    //             &transfer_to_from,
    //             &[
    //                 ctx.accounts.matched.to_account_info(),
    //                 ctx.accounts.from.to_account_info(),
    //             ],
    //         )?;
    //         anchor_lang::solana_program::program::invoke(
    //             &transfer_to_to,
    //             &[
    //                 ctx.accounts.matched.to_account_info(),
    //                 ctx.accounts.to.to_account_info(),
    //             ],
    //         )?;
    //     } else {
    //         // Refund only the requester's stake
    //         let transfer_to_requester = anchor_lang::solana_program::system_instruction::transfer(
    //             &ctx.accounts.matched.to_account_info().key(),
    //             &ctx.accounts.from.key(),
    //             200_000_000,
    //         );

    //         anchor_lang::solana_program::program::invoke(
    //             &transfer_to_requester,
    //             &[
    //                 ctx.accounts.matched.to_account_info(),
    //                 ctx.accounts.from.to_account_info(),
    //             ],
    //         )?;
    //     }

    //     **ctx.accounts.signer.to_account_info().lamports.borrow_mut() +=
    //         ctx.accounts.matched.to_account_info().lamports();
    //     **ctx.accounts.matched.to_account_info().lamports.borrow_mut() = 0;

    //     Ok(())
    // }
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

// #[derive(Accounts)]
// pub struct RequestConnection<'info> {
//     #[account(
//         init,
//         payer = signer,
//         space = 8 + 96,
//         seeds = [b"matched", signer.key().as_ref(), to.key().as_ref()],
//         bump
//     )]
//     pub matched: Account<'info, Matched>,
//     #[account(
//         mut,
//         seeds = [b"user", signer.key().as_ref()],
//         bump
//     )]
//     pub user: Account<'info, User>,
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     #[account(
//         mut,
//         seeds = [b"user", to.key().as_ref()],
//         bump
//     )]
//     pub to_user: Account<'info, User>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub to: UncheckedAccount<'info>,
//     pub system_program: Program<'info, System>,
// }

#[derive(Accounts)]
pub struct RequestConnection<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 72, // Adjusted space
        seeds = [b"matched", signer.key().as_ref(), to.key().as_ref()],
        bump
    )]
    pub matched: Account<'info, Matched>,
    // #[account(
    //     mut,
    //     seeds = [b"user", signer.key().as_ref()],
    //     bump
    // )]
    // pub user: Account<'info, User>,
    #[account(mut)]
    pub signer: Signer<'info>,
    // #[account(
    //     mut,
    //     seeds = [b"user", to.key().as_ref()],
    //     bump
    // )]
    // pub to_user: Account<'info, User>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub to: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptConnection<'info> {
    #[account(
        mut,
        seeds = [b"matched", matched.from.as_ref(), signer.key().as_ref()],
        bump
    )]
    pub matched: Account<'info, Matched>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RejectConnection<'info> {
    #[account(
        mut,
        close = from,
        seeds = [b"matched", matched.from.as_ref(), matched.to.as_ref()],
        bump
    )]
    pub matched: Account<'info, Matched>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub from: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
}

// #[derive(Accounts)]
// pub struct WithdrawStake<'info> {
//     #[account(
//         mut,
//         close = signer,
//         seeds = [b"matched", matched.from.as_ref(), matched.to.as_ref()],
//         bump
//     )]
//     pub matched: Account<'info, Matched>,
//     #[account(mut)]
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub from: UncheckedAccount<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub to: UncheckedAccount<'info>,
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct UpdateProfile<'info> {
//     #[account(
//         mut,
//         seeds = [b"user", signer.key().as_ref()],
//         bump
//     )]
//     pub user: Account<'info, User>,
//     #[account(mut)]
//     pub signer: Signer<'info>,
// }

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user_pubkey: Pubkey,
    #[max_len(20)]
    pub name: String,
    pub req_count: u32,
    pub req_sent: u32,
}

#[account]
pub struct Matched {
    pub from: Pubkey,
    pub to: Pubkey,
    pub is_matched: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized action.")]
    Unauthorized,
}
