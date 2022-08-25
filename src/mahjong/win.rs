use super::*;

pub fn detect_winning(ctx: &HandContext) -> Vec<YakuContext> {
    let mut output: Vec<YakuContext> = Vec::new();
    let hand_tiles = &ctx.hand_tiles;
    let mut hand = TileTable::default();
    for tile in hand_tiles {
        hand[tile.0 .0][tile.0 .1] += 1;
    }

    output
}
