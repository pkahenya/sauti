//! Sauti v0: contributor registry and clip fingerprint log.
//!
//! Each contributor gets an account derived from their wallet, holding a
//! SHA-256 fingerprint of the consent they gave in the app. Each recorded
//! clip is logged as an on-chain event carrying the fingerprint of its audio
//! and its language. Audio and personal details never go on-chain.
//!
//! Rent and fees can be paid by a separate wallet (the Sauti relayer), so
//! contributors never need SOL.

use anchor_lang::prelude::*;

// Placeholder until the first devnet deploy. Replace with the deployed
// program ID (or run `anchor keys sync`).
declare_id!("11111111111111111111111111111111");

#[program]
pub mod sauti {
    use super::*;

    /// Creates the contributor's account with the fingerprint of their consent.
    pub fn register_contributor(
        ctx: Context<RegisterContributor>,
        consent_hash: [u8; 32],
    ) -> Result<()> {
        let contributor = &mut ctx.accounts.contributor;
        contributor.authority = ctx.accounts.authority.key();
        contributor.consent_hash = consent_hash;
        contributor.clips = 0;
        contributor.registered_at = Clock::get()?.unix_timestamp;
        contributor.bump = ctx.bumps.contributor;
        Ok(())
    }

    /// Logs the fingerprint and language of a recorded clip.
    pub fn submit_clip(
        ctx: Context<SubmitClip>,
        clip_hash: [u8; 32],
        language: Language,
    ) -> Result<()> {
        let contributor = &mut ctx.accounts.contributor;
        contributor.clips = contributor
            .clips
            .checked_add(1)
            .ok_or(SautiError::ClipCountOverflow)?;
        emit!(ClipSubmitted {
            contributor: contributor.authority,
            clip_hash,
            language,
            index: contributor.clips,
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterContributor<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Contributor::INIT_SPACE,
        seeds = [b"contributor", authority.key().as_ref()],
        bump
    )]
    pub contributor: Account<'info, Contributor>,
    /// The contributor's own wallet.
    pub authority: Signer<'info>,
    /// Pays rent for the new account. Usually the Sauti relayer.
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitClip<'info> {
    #[account(
        mut,
        seeds = [b"contributor", authority.key().as_ref()],
        bump = contributor.bump,
        has_one = authority
    )]
    pub contributor: Account<'info, Contributor>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Contributor {
    pub authority: Pubkey,
    pub consent_hash: [u8; 32],
    pub clips: u64,
    pub registered_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    Sheng,
    Kikuyu,
    Dholuo,
    Luhya,
    Kalenjin,
    Somali,
    KenyanEnglish,
}

#[event]
pub struct ClipSubmitted {
    pub contributor: Pubkey,
    pub clip_hash: [u8; 32],
    pub language: Language,
    pub index: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum SautiError {
    #[msg("The contributor's clip count cannot go higher")]
    ClipCountOverflow,
}
