use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("63E3knaFYs7ZSPuDiUkSwmZNAgPoD7J3y9Qiay5otRdZ");

#[program]
pub mod elfo {
    use super::*;
}
