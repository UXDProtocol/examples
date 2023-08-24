pub mod instructions;

use crate::instructions::*;
use anchor_lang::prelude::*;

#[cfg(feature = "development")]
declare_id!("4zFsVBdd3q6jNZWZ3DSSFTvm7yS8TnwfEM2Eq4Gzdeb4");

#[program]
pub mod solana_cpi {
    use super::*;

    pub fn mint_with_identity_depository_cpi(
        ctx: Context<MintWithIdentityDepositoryCpi>,
        collateral_amount: u64,
    ) -> Result<()> {
        instructions::mint_with_identity_depository_cpi(ctx, collateral_amount)
    }

    pub fn redeem_from_identity_depository_cpi(
        ctx: Context<RedeemFromIdentityDepositoryCpi>,
        redeemable_amount: u64,
    ) -> Result<()> {
        instructions::redeem_from_identity_depository_cpi(ctx, redeemable_amount)
    }
}
