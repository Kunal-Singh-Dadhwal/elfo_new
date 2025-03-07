use std::convert::TryInto;

use crate::{
    constants::{CANCELLATION_DELEGATED_AMOUNT_NOT_ENOUGH, CANCELLATION_DELEGATION_REVOKED, CANCELLATION_INSUFFICIENT_AMOUNT},
    error::ErrorCode,
    state::*
}

use anchor_lang::prelude::*;
use anchor_spl::{
    mint,
    token::{self, spl_token::state::Account, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct TriggerPayment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = subscriber_payement_account.mint = mint.key() @ ErrorCode::InvalidMint
    )]
    pub subscriber_payement_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"protocol_signer"],
        bump = protocol_signer.bump,
    )]
    pub protocol_signer: Box<Account<'info, ProtocolSigner>>,

    #[account(
        mut,
        has_one = subscription_plan,
        has_one = subscriber,
        constraint = subscription.has_aldready_been_initialized @ ErrorCode::SubscriberNotInitialized,
        constraint = subscription.is_active @ ErrorCode::SubscriptionNotSubscribed
    )]
    pub subscription: Box<Account<'info, Subscription>>,

    #[account(
        constraint = subscriber.has_aldready_been_initialized @ ErrorCode::SubscriberNotInitialized,
        has_one = subscriber_payement_account,
    )]
    pub subscriber: Box<Account<'info, Subscriber>>,


}