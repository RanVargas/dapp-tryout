use anchor_lang::prelude::*;
use anchor_lang::error;

declare_id!("Hh8eRpditCZooxe1DorCsinz5gb5CtoyYdgksMgseCun");

#[program]
pub mod dapp_thrid_try {

    use super::*;

    pub fn adding_task(ctx: Context<AddingTask>, text: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        if text.chars().count() > 400 {

            return Err(Error::from(ErrorCode::TextTooLong));
        }

        task.author = *author.key;
        task.is_done = false;
        task.created_at = clock.unix_timestamp;
        task.updated_at = clock.unix_timestamp;
        task.text = text;

        Ok(())
    }

    pub fn updating_task(ctx: Context<UpdatingTask>, is_done: bool) -> Result<()>{
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();
        task.author = *author.key;
        task.is_done = is_done;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()>{
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();
        task.author = *author.key;
        task.is_done = true;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct Task{
    pub author: Pubkey,
    pub is_done: bool,
    pub text: String,
    pub created_at: i64,
    pub updated_at: i64,
}
const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const BOOL_LENGTH: usize = 1;
const TEXT_LENGTH: usize = 4 + 400 * 4;
const TIMESTAMP_LENGTH: usize = 8;

impl Task{
    const LEN: usize = DISCRIMINATOR +
        PUBLIC_KEY_LENGTH +
        BOOL_LENGTH +
        TEXT_LENGTH +
        TIMESTAMP_LENGTH +
        TIMESTAMP_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("The text is too long")]
    TextTooLong,
}

#[derive(Accounts)]
pub struct UpdatingTask<'info>{
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>
}

pub struct DeletingTask<'info>{
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>
}