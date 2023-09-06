use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RedeemFromIdentityDepositoryCpi<'info> {
    pub user: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// The UXD Controller PDA
    #[account(
        mut,
        seeds = [b"CONTROLLER"],
        bump = controller.bump,
        constraint = controller.identity_depository == identity_depository.key(),
        has_one = redeemable_mint
    )]
    pub controller: Account<'info, uxd_cpi::Controller>,

    #[account(mut)]
    pub redeemable_mint: AccountInfo<'info>,
    pub collateral_mint: AccountInfo<'info>,

    /// Token account for user redeemable (redeemable mint)
    pub user_redeemable: Account<'info, TokenAccount>,
    /// Token account for user collateral (collateral mint)
    pub user_collateral: Account<'info, TokenAccount>,

    /// The UXD Identity Depository PDA (a no transformation depository)
    pub identity_depository: AccountInfo<'info>,
    pub identity_depository_collateral_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub uxd_program: Program<'info, uxd_cpi::program::Uxd>,
}

pub fn redeem_from_identity_depository_cpi(
    ctx: Context<RedeemFromIdentityDepositoryCpi>,
    redeemable_amount: u64,
) -> Result<()> {
    let controller_pda_signer: &[&[&[u8]]] = &[&[b"CONTROLLER", &[ctx.accounts.controller.bump]]];

    uxd_cpi::cpi::redeem_from_identity_depository(
        ctx.accounts
            .into_redeem_from_identity_depository_context()
            .with_signer(controller_pda_signer),
        redeemable_amount,
    )
}

// Into functions
impl<'info> RedeemFromIdentityDepositoryCpi<'info> {
    pub fn into_redeem_from_identity_depository_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, uxd_cpi::cpi::accounts::RedeemFromIdentityDepository<'info>>
    {
        let cpi_accounts = uxd_cpi::cpi::accounts::RedeemFromIdentityDepository {
            authority: self.controller.to_account_info(),
            user: self.user.to_account_info(),
            payer: self.payer.to_account_info(),
            controller: self.controller.to_account_info(),
            redeemable_mint: self.redeemable_mint.to_account_info(),
            user_redeemable: self.user_redeemable.to_account_info(),
            user_collateral: self.user_collateral.to_account_info(),
            depository: self.identity_depository.to_account_info(),
            collateral_vault: self.identity_depository_collateral_vault.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_program = self.uxd_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
