use super::*;

#[derive(Debug)]
pub struct DoraInfo {
    pub dora_fan_sum: usize,
    pub dora_fan: usize,
    pub ura_dora_fan: usize,
    pub aka_dora_fan: usize,
    pub nuki_dora_fan: usize,
}

impl DoraInfo {
    pub fn new(
        dora_fan: usize,
        ura_dora_fan: usize,
        aka_dora_fan: usize,
        nuki_dora_fan: usize,
    ) -> Self {
        Self {
            dora_fan_sum: dora_fan + ura_dora_fan + aka_dora_fan + nuki_dora_fan,
            dora_fan,
            ura_dora_fan,
            aka_dora_fan,
            nuki_dora_fan,
        }
    }
}

pub fn calc_dora(ctx: &HandContext) -> DoraInfo {
    let mut dora_fan: usize = 0;
    let mut ura_dora_fan: usize = 0;
    let mut aka_dora_fan: usize = 0;
    let mut nuki_dora_fan: usize = 0;

    let mut hand = TileTable::default();
    {
        let TileWithDora(agari_tile, d) = ctx.agari_tile;
        hand[agari_tile.0][agari_tile.1] += 1;
        aka_dora_fan += d;
    }
    for TileWithDora(tile, d) in &ctx.hand_tiles {
        hand[tile.0][tile.1] += 1;
        aka_dora_fan += d;
    }
    for fuuro in &ctx.fuuro {
        let Fuuro(_, v) = fuuro;
        for TileWithDora(tile, d) in v {
            hand[tile.0][tile.1] += 1;
            aka_dora_fan += d;
        }
    }
    for TileWithDora(tile, d) in &ctx.nuki_dora {
        hand[tile.0][tile.1] += 1;
        nuki_dora_fan += d;
    }

    for tile in &ctx.dora {
        dora_fan += hand[tile.0][tile.1];
    }
    for tile in &ctx.ura_dora {
        ura_dora_fan += hand[tile.0][tile.1];
    }

    DoraInfo::new(dora_fan, ura_dora_fan, aka_dora_fan, nuki_dora_fan)
}
