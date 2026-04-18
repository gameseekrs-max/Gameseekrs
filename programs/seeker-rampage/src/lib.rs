#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, FreezeAccount};

declare_id!("7N3D97mmgFXiKN9LrHyQZKcpVBzukh34DDYpnSqaoTov");

// =============================================
// OFFICIAL DEPLOYER / FUNDING WALLET
// =============================================

// ── DEVNET (current) ──
pub const DEPLOYER_AUTHORITY: Pubkey = pubkey!("4oisjFF93q8bc8XHGRzVMLgyfDo4QgVYkuHhGJXKBkMt");

// ── MAINNET (uncomment when ready to deploy) ──
// pub const DEPLOYER_AUTHORITY: Pubkey = pubkey!("YOUR_MULTISIG_MAINNET_ADDRESS_HERE");

// =============================================
// ERRORS (moved to top - exactly like lib_example.rs)
// =============================================
#[error_code]
pub enum ErrorCode {
    #[msg("Must reach Level 3")]
    MustReachLevel3,
    #[msg("Must own a Legend SBT")]
    MustOwnLegendSBT,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("All 1000 free Legend SBTs have been minted")]
    AllSBTsMinted,
    #[msg("Daily rewards are currently disabled by admin")]
    DailyRewardsDisabled,
    #[msg("You already claimed today — come back in 24h")]
    AlreadyClaimedToday,
    #[msg("Label too long (max 64 chars)")]
    LabelTooLong,
    #[msg("Cannot transfer to self")]
    InvalidTransfer,
    #[msg("Insufficient funds in reward pool")]
    InsufficientRewardPool,
}

#[program]
pub mod seeker_rampage {
    use super::*;

    // =============================================
    // PLAY GAME
    // =============================================
    pub fn play_game(ctx: Context<PlayGame>, score: u32, level: u8) -> Result<()> {
        let user_state = &mut ctx.accounts.user_state;
        if score > user_state.high_score {
            user_state.high_score = score;
        }
        user_state.highest_level = level.max(user_state.highest_level);
        msg!("🎮 High score saved: {} | Highest level: {}", score, user_state.highest_level);
        Ok(())
    }

    // =============================================
    // MINT LEGEND SBT (first 1000 free)
    // =============================================
    pub fn mint_legend_sbt(ctx: Context<MintLegendSbt>, label1: String, label2: String) -> Result<()> {
        require!(ctx.accounts.user_state.highest_level >= 3, ErrorCode::MustReachLevel3);
        require!(label1.len() <= 64, ErrorCode::LabelTooLong);
        require!(label2.len() <= 64, ErrorCode::LabelTooLong);

        let config = &mut ctx.accounts.legend_config;
        require!(config.minted_legends < 1000, ErrorCode::AllSBTsMinted);

        let sbt_cpi = token::MintTo {
            mint: ctx.accounts.sbt_mint.to_account_info(),
            to: ctx.accounts.user_sbt_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::mint_to(CpiContext::new(ctx.accounts.token_program.key(), sbt_cpi), 1)?;

        let freeze_cpi = token::FreezeAccount {
            account: ctx.accounts.user_sbt_account.to_account_info(),
            mint: ctx.accounts.sbt_mint.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::freeze_account(CpiContext::new(ctx.accounts.token_program.key(), freeze_cpi))?;

        let status = &mut ctx.accounts.user_legend_status;
        status.owner = ctx.accounts.user.key();
        status.is_legend = true;
        status.label1 = label1;
        status.label2 = label2;
        status.last_claim = 0;
        status.claims_today = 0;
        status.bump = ctx.bumps.user_legend_status;

        config.minted_legends += 1;
        msg!("🏆 Legend SBT #{} minted FREE! ({} slots left)", config.minted_legends, 1000 - config.minted_legends);
        Ok(())
    }

    // =============================================
    // MINT NORMAL SBT (paid after 1000)
    // =============================================
    pub fn mint_normal_sbt(ctx: Context<MintNormalSbt>, label1: String, label2: String) -> Result<()> {
        require!(label1.len() <= 64, ErrorCode::LabelTooLong);
        require!(label2.len() <= 64, ErrorCode::LabelTooLong);

        let amount: u64 = 1_000_000_000;
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.key(),
                Transfer {
                    from: ctx.accounts.user_token.to_account_info(),
                    to: ctx.accounts.reward_pool_token.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;

        let sbt_cpi = token::MintTo {
            mint: ctx.accounts.sbt_mint.to_account_info(),
            to: ctx.accounts.user_sbt_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::mint_to(CpiContext::new(ctx.accounts.token_program.key(), sbt_cpi), 1)?;

        let freeze_cpi = token::FreezeAccount {
            account: ctx.accounts.user_sbt_account.to_account_info(),
            mint: ctx.accounts.sbt_mint.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::freeze_account(CpiContext::new(ctx.accounts.token_program.key(), freeze_cpi))?;

        msg!("✅ Normal SBT minted for 1 SKR (self-funding reward pool)");
        Ok(())
    }

    // =============================================
    // INITIALIZE POOLS
    // =============================================
    pub fn initialize_reward_pools(ctx: Context<InitializeRewardPools>) -> Result<()> {
        let reward_pool = &mut ctx.accounts.reward_pool;
        let bootstrap_pool = &mut ctx.accounts.bootstrap_pool;
        reward_pool.authority = DEPLOYER_AUTHORITY;
        reward_pool.bump = ctx.bumps.reward_pool;
        bootstrap_pool.authority = DEPLOYER_AUTHORITY;
        bootstrap_pool.bump = ctx.bumps.bootstrap_pool;
        msg!("✅ Reward + Bootstrap pools initialized with official deployer");
        Ok(())
    }

    pub fn initialize_legend_config(ctx: Context<InitializeLegendConfig>) -> Result<()> {
        let config = &mut ctx.accounts.legend_config;
        config.minted_legends = 0;
        config.authority = DEPLOYER_AUTHORITY;
        config.bump = ctx.bumps.legend_config;
        msg!("✅ LegendConfig initialized with official deployer");
        Ok(())
    }

    // =============================================
    // CLAIM DAILY + ADMIN
    // =============================================
    pub fn claim_daily_skr(ctx: Context<ClaimDailySKR>) -> Result<()> {
        let clock = Clock::get()?;
        require!(ctx.accounts.program_config.daily_rewards_enabled, ErrorCode::DailyRewardsDisabled);
        require!(ctx.accounts.user_legend_status.is_legend, ErrorCode::MustOwnLegendSBT);
        require!(ctx.accounts.user_state.highest_level >= 3, ErrorCode::MustReachLevel3);

        let last_claim = ctx.accounts.user_legend_status.last_claim;
        require!(clock.unix_timestamp - last_claim > 86400, ErrorCode::AlreadyClaimedToday);

        let amount: u64 = 50_000_000;

        if ctx.accounts.bootstrap_pool_token.amount >= amount {
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.key(),
                    Transfer { from: ctx.accounts.bootstrap_pool_token.to_account_info(), to: ctx.accounts.user_skr_account.to_account_info(), authority: ctx.accounts.bootstrap_pool.to_account_info() },
                    &[&[b"bootstrap-pool", &[ctx.bumps.bootstrap_pool]]],
                ),
                amount,
            )?;
        } else {
            require!(ctx.accounts.reward_pool_token.amount >= amount, ErrorCode::InsufficientRewardPool);
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.key(),
                    Transfer { from: ctx.accounts.reward_pool_token.to_account_info(), to: ctx.accounts.user_skr_account.to_account_info(), authority: ctx.accounts.reward_pool.to_account_info() },
                    &[&[b"reward-pool", &[ctx.bumps.reward_pool]]],
                ),
                amount,
            )?;
        }

        let status = &mut ctx.accounts.user_legend_status;
        status.last_claim = clock.unix_timestamp;
        status.claims_today += 1;
        Ok(())
    }

    pub fn toggle_daily_rewards(ctx: Context<ToggleDailyRewards>, enabled: bool) -> Result<()> {
        require_keys_eq!(ctx.accounts.authority.key(), DEPLOYER_AUTHORITY, ErrorCode::Unauthorized);
        let config = &mut ctx.accounts.program_config;
        config.daily_rewards_enabled = enabled;
        msg!("🔧 Daily rewards toggled to: {}", if enabled { "ENABLED" } else { "DISABLED" });
        Ok(())
    }

    pub fn transfer_legend_sbt(ctx: Context<TransferLegendSbt>, price: u64) -> Result<()> {
        require_keys_eq!(ctx.accounts.user_legend_status.owner, ctx.accounts.seller.key(), ErrorCode::Unauthorized);
        require_keys_neq!(ctx.accounts.seller.key(), ctx.accounts.buyer.key(), ErrorCode::InvalidTransfer);
        require!(ctx.accounts.user_legend_status.is_legend, ErrorCode::MustOwnLegendSBT);

        let royalty = price / 10;
        let to_seller = price - royalty;

        let pool_balance = ctx.accounts.reward_pool_token.amount;
        let is_big_pool = pool_balance > 1_000_000_000_000;

        if is_big_pool {
            let half = royalty / 2;
            token::transfer(CpiContext::new(ctx.accounts.token_program.key(), Transfer { from: ctx.accounts.buyer_token.to_account_info(), to: ctx.accounts.deployer_token.to_account_info(), authority: ctx.accounts.buyer.to_account_info() }), half)?;
            token::transfer(CpiContext::new(ctx.accounts.token_program.key(), Transfer { from: ctx.accounts.buyer_token.to_account_info(), to: ctx.accounts.reward_pool_token.to_account_info(), authority: ctx.accounts.buyer.to_account_info() }), half)?;
            msg!("🏆 Big pool split: 50% to deployer + 50% to pool");
        } else {
            token::transfer(CpiContext::new(ctx.accounts.token_program.key(), Transfer { from: ctx.accounts.buyer_token.to_account_info(), to: ctx.accounts.reward_pool_token.to_account_info(), authority: ctx.accounts.buyer.to_account_info() }), royalty)?;
            msg!("🏆 Bootstrap royalty to pool");
        }

        token::transfer(CpiContext::new(ctx.accounts.token_program.key(), Transfer { from: ctx.accounts.buyer_token.to_account_info(), to: ctx.accounts.seller_token.to_account_info(), authority: ctx.accounts.buyer.to_account_info() }), to_seller)?;

        let status = &mut ctx.accounts.user_legend_status;
        status.owner = ctx.accounts.buyer.key();
        Ok(())
    }

    // =============================================
    // ACCOUNTS & CONTEXTS
    // =============================================
    #[account]
    pub struct UserState { pub high_score: u32, pub highest_level: u8, }

    #[account]
    pub struct LegendConfig { pub minted_legends: u64, pub authority: Pubkey, pub bump: u8, }

    #[account]
    pub struct ProgramConfig { pub daily_rewards_enabled: bool, pub authority: Pubkey, pub bump: u8, }

    #[account]
    pub struct LegendStatus { pub owner: Pubkey, pub is_legend: bool, pub label1: String, pub label2: String, pub last_claim: i64, pub claims_today: u8, pub bump: u8, }

    #[account]
    pub struct RewardPool { pub authority: Pubkey, pub bump: u8, }

    #[account]
    pub struct BootstrapPool { pub authority: Pubkey, pub bump: u8, }

    #[derive(Accounts)]
    pub struct PlayGame<'info> {
        #[account(mut)] pub user_state: Account<'info, UserState>,
        pub user: Signer<'info>,
    }

    #[derive(Accounts)]
    #[instruction(label1: String, label2: String)]
    pub struct MintLegendSbt<'info> {
        #[account(mut)] pub legend_config: Account<'info, LegendConfig>,
        #[account(mut)] pub sbt_mint: Account<'info, Mint>,
        #[account(mut)] pub user_sbt_account: Account<'info, TokenAccount>,
        #[account(init, payer = user, space = 8 + 32 + 1 + 4 + 64 + 4 + 64 + 8 + 1 + 1, seeds = [b"legend_status", user.key().as_ref()], bump)]
        pub user_legend_status: Account<'info, LegendStatus>,
        #[account(mut)] pub user_state: Account<'info, UserState>,
        #[account(mut)] pub user: Signer<'info>,
        pub authority: Signer<'info>,
        pub token_program: Program<'info, Token>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    #[instruction(label1: String, label2: String)]
    pub struct MintNormalSbt<'info> {
        #[account(mut)] pub sbt_mint: Account<'info, Mint>,
        #[account(mut)] pub user_sbt_account: Account<'info, TokenAccount>,
        #[account(mut)] pub user_token: Account<'info, TokenAccount>,
        #[account(mut)] pub reward_pool_token: Account<'info, TokenAccount>,
        #[account(mut)] pub user: Signer<'info>,
        pub authority: Signer<'info>,
        pub token_program: Program<'info, Token>,
    }

    #[derive(Accounts)]
    pub struct InitializeRewardPools<'info> {
        #[account(init, payer = user, space = 8 + 32 + 1, seeds = [b"reward-pool"], bump)]
        pub reward_pool: Account<'info, RewardPool>,
        #[account(init, payer = user, space = 8 + 32 + 1, seeds = [b"bootstrap-pool"], bump)]
        pub bootstrap_pool: Account<'info, BootstrapPool>,
        #[account(mut)] pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct InitializeLegendConfig<'info> {
        #[account(init, payer = user, space = 8 + 8 + 32 + 1, seeds = [b"legend-config"], bump)]
        pub legend_config: Account<'info, LegendConfig>,
        #[account(mut)] pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct ClaimDailySKR<'info> {
        #[account(mut)] pub user_legend_status: Account<'info, LegendStatus>,
        #[account(mut, seeds = [b"bootstrap-pool"], bump)] pub bootstrap_pool: Account<'info, BootstrapPool>,
        #[account(mut, associated_token::mint = skr_mint, associated_token::authority = bootstrap_pool)] pub bootstrap_pool_token: Account<'info, TokenAccount>,
        #[account(mut, seeds = [b"reward-pool"], bump)] pub reward_pool: Account<'info, RewardPool>,
        #[account(mut, associated_token::mint = skr_mint, associated_token::authority = reward_pool)] pub reward_pool_token: Account<'info, TokenAccount>,
        #[account(mut)] pub user_skr_account: Account<'info, TokenAccount>,
        #[account(mut)] pub program_config: Account<'info, ProgramConfig>,
        #[account(mut)] pub user_state: Account<'info, UserState>,
        pub user: Signer<'info>,
        pub authority: Signer<'info>,
        pub token_program: Program<'info, Token>,
        pub skr_mint: Account<'info, Mint>,
    }

    #[derive(Accounts)]
    pub struct ToggleDailyRewards<'info> {
        #[account(mut)] pub program_config: Account<'info, ProgramConfig>,
        pub authority: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct TransferLegendSbt<'info> {
        #[account(mut)] pub user_legend_status: Account<'info, LegendStatus>,
        #[account(mut)] pub buyer_token: Account<'info, TokenAccount>,
        #[account(mut)] pub seller_token: Account<'info, TokenAccount>,
        #[account(mut)] pub reward_pool_token: Account<'info, TokenAccount>,
        #[account(mut)] pub deployer_token: Account<'info, TokenAccount>,
        pub seller: Signer<'info>,
        pub buyer: Signer<'info>,
        pub token_program: Program<'info, Token>,
    }
}