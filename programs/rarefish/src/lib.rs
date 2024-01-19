use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod rarefish {
    use super::*;

    pub fn initalize_rare_fish(ctx: Context<InitializeRareFish>) -> Result<()> {


        Ok(())
    }

    pub fn new_fish(ctx: Context<NewFish>) -> Result<()> {

        Ok(())
    }

    pub fn breed_fish(ctx: Context<BreedFish>) -> Result<()> {

        Ok(())
    }

    pub fn toggle_fish_market(ctx: Context<ToggleFishMarket>) -> Result<()> {

        Ok(())
    }

    pub fn increase_market_supply(ctx: Context<IncreaseMarketSupply>) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRareFish {

}

#[derive(Accounts)]
pub struct NewFish {

}

#[derive(Accounts)]
pub struct BreedFish {

}

#[derive(Accounts)]
pub struct ToggleFishMarket {

}

#[derive(Accounts)]
pub struct IncreaseMarketSupply {

}

#[account]
pub struct FishMarket {
    authority: Pubkey,
    max_fish: u64,
    current_fish: u64,
    open: bool
}

#[account]
pub struct RareFish {
    authority: Pubkey,
    seed: u64,
    token: Pubkey,
    gender: bool,
    father: Pubkey,
    mother: Pubkey
}