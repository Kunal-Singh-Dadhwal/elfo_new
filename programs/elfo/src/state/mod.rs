use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
#[derive(InitSpace)]
pub struct elfo {
    pub bump: u8,
    pub aldready_initialized: bool,
    pub authority: Pubkey,
    #[max_len(MAXIMUM_CREATOR_ACCOUNTS)]
    pub creator_accounts: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Creator {
    pub bump: u8,
    pub has_aldready_initialized: bool,
    pub authority: Pubkey,
    #[max_len(64)]
    pub name: String,
    #[max_len(64)]
    pub data_id: String,
    pub subscription_plan: Pubkey,
    #[max_len(MAXIMUM_POSTS_PER_CREATOR)]
    pub posts: Vec<Pubkey>,
    pub last_post_index: i64,
}

pub struct CreatorPost {}
