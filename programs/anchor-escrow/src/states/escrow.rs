use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub bump: u8,
    pub initializer: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 // Discriminator (u64)
      + 8 // seed
      + 1 // bump
      + 32 // initializer
      + 32 // mint_a
      + 32 // mint_b
      + 8 // initializer_amount
      + 8; // taker_amount
}