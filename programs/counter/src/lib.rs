use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod counter {
    use super::*;
    pub fn start(ctx: Context<Start>, value: i32) -> ProgramResult {
        ctx.accounts.counter.value = value;
        Ok(())
    }

    pub fn incr(ctx: Context<Incr>, value: i32) -> ProgramResult {
        ctx.accounts.counter.value += value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(init, payer = user, space = 32)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Incr<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>
}

#[account]
pub struct Counter {
    value: i32,
}

