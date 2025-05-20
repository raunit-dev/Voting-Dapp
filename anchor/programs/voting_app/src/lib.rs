#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod voting_app {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>,poll_id: u64,description: String,poll_start: u64,poll_end: u64) -> Result<()> {
       ctx.accounts.poll.poll_id = poll_id;
        ctx.accounts.poll.poll_start = poll_start;
        ctx.accounts.poll.poll_end = poll_end;
        ctx.accounts.poll.description = description;
        ctx.accounts.poll.candidate_amount = 0;
        Ok(())
    }
    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        candidate_name: String,
        poll_id: u64
    ) -> Result<()> {
        ctx.accounts.candidate_account.candidate_name = candidate_name;
        ctx.accounts.candidate_account.candidate_vote = 0; // Initialize votes to 0
        ctx.accounts.poll.candidate_amount += 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64,description:String)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [poll_id.to_le_bytes().as_ref()],
        space = 8 + Poll::INIT_SPACE,
        bump
    )]
    pub poll: Account<'info,Poll>,
    pub system_program: Program<'info,System>
}


#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct InitializeCandidate<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info,Poll>,

    #[account(
        init,
        payer = signer,
        seeds = [poll_id.to_le_bytes().as_ref(),candidate_name.as_ref()],
        space = 8 + Candidate::INIT_SPACE,
        bump
    )]
    pub candidate_account: Account<'info,Candidate>,

    pub system_program: Program<'info,System>,
}



#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(50)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(65)]
    pub candidate_name: String,
    pub candidate_vote: u64
}