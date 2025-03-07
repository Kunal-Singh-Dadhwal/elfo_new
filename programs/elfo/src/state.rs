use crate::constants::{
    MAXIMUM_NODES, MAXIMUM_SUBSCRIPTIONS_PER_PLAN, MAXIMUM_SUBSCRIPTIONS_PER_USER,
    MAXIMUM_SUBSCRIPTION_PLANS, MAXIMUM_SUBSCRIPTION_PLAN_PER_AUTHOR,
};
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Subscriber {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,
    pub subscriber_payment_account: Pubkey,
    #[max_len(MAXIMUM_SUBSCRIPTIONS_PER_USER)]
    pub subscription_accounts: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Subscription {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub subscriber: Pubkey,
    pub subscription_plan: Pubkey,
    pub is_active: bool,
    pub is_cancelled: bool,
    pub cancellation_reason: i8,

    pub last_payment_timestamp: i64,
    pub next_payment_timestamp: i64,
}

#[account]
#[derive(InitSpace)]
pub struct SubscriptionPlan {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    #[max_len(100)]
    pub plan_name: String,
    pub subscription_plan_author: Pubkey,
    pub subscription_plan_payment_account: Pubkey,
    pub amount: i64,
    pub frequency: i64,
    pub is_active: bool,
    pub fee_percentage: i8,
    #[max_len(MAXIMUM_SUBSCRIPTIONS_PER_PLAN)]
    pub subscription_accounts: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct SubscriptionPlanAuthor {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,
    #[max_len(MAXIMUM_SUBSCRIPTION_PLAN_PER_AUTHOR)]
    pub subscription_plan_accounts: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Protocol {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,
    #[max_len(MAXIMUM_SUBSCRIPTION_PLANS)]
    pub subscription_plan_accounts: Vec<Pubkey>,
    #[max_len(MAXIMUM_NODES)]
    pub registered_nodes: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct ProtocolSigner {
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Node {
    pub bump: u8,
    pub is_registered: bool,
    pub authority: Pubkey,
    pub node_payment_wallet: Pubkey,
    pub node_payment_account: Pubkey,
}
