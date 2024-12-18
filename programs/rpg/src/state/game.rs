// ----------- ACCOUNTS ----------
 
// Inside `state/game.rs`
use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct Game {
    pub game_master: Pubkey,
    pub treasury: Pubkey,
    pub action_points_collected: u64,
    pub game_config: GameConfig,
}
 
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct GameConfig {
    pub max_items_per_player: u8
}