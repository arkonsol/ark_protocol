// use anchor_lang::prelude::*;

// #[account]
// pub struct EpochConfig {
//     pub bump: u8,
//     pub create_key: Pubkey,
//     pub deployer: Pubkey,
//     pub epoch_duration_seconds: u64,
//     pub epoch_index: u32,
//     pub epochs: Vec<Pubkey>,
// }

// #[account]
// pub struct Epoch {
//     pub bump: u8,
//     pub epoch_config: Pubkey,
//     pub index: u32,
//     pub created_at: i64,
//     pub expired_at: i64,
//     pub minimum_witnesses_for_claim: u8,
//     pub witnesses: Vec<Witness>,
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
// pub struct Witness {
//     pub address: String,
//     pub url: String,
// }

// impl EpochConfig {
//     pub const SPACE: usize = 8 + // discriminator
//         1 + // bump
//         32 + // create_key
//         32 + // deployer
//         8 + // epoch_duration_seconds
//         4 + // epoch_index
//         4 + (32 * 10); // epochs (assuming max 10 epochs)
// }

// impl Epoch {
//     pub const SPACE: usize = 8 + // discriminator
//         1 + // bump
//         32 + // epoch_config
//         4 + // index
//         8 + // created_at
//         8 + // expired_at
//         1 + // minimum_witnesses_for_claim
//         4 + (64 * 10); // witnesses (assuming max 10 witnesses, 32 bytes for address and 32 for url)
// }