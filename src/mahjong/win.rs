use super::*;

pub fn detect_winning(ctx: &HandContext) -> Vec<YakuContext> {
    let fuuro_cnt = 3 * &ctx.fuuro.len();
    let hand_tiles = &ctx.hand_tiles;
    if fuuro_cnt + hand_tiles.len() != 13 {
        return vec![];
    }

    let tsumo = ctx.tsumo;
    let TileWithDora(agari_tile, _) = ctx.agari_tile;
    let yaku_flags = ctx.yaku_flags;

    let mut output: Vec<YakuContext> = Vec::new();

    let mut hand = TileTable::default();
    for TileWithDora(tile, _) in hand_tiles {
        hand[tile.0][tile.1] += 1;
    }
    hand[agari_tile.0][agari_tile.1] += 1;

    // 七対子と国士無双の判定
    let mut pairs: Vec<Tile> = Vec::new();
    let mut kokushi_flg = true;
    for i in 0..3 {
        for j in 1..=9 {
            let n = hand[i][j];
            if n == 2 {
                pairs.push(Tile(i, j));
            }
            if n != 0 {
                kokushi_flg = false;
            }
        }
    }
    for j in 1..=7 {
        let n = hand[TZ][j];
        if n == 2 {
            pairs.push(Tile(TZ, j));
        }
        if n == 0 || n >= 3 {
            kokushi_flg = false;
        }
    }
    // 国士無双の場合は国士無双のみ追加して終了
    if kokushi_flg {
        output.push(YakuContext::new(
            hand,
            YakuForm::KokushiMusou,
            agari_tile,
            tsumo,
            yaku_flags,
        ));
        return output;
    }
    // 七対子の場合は4面子1雀頭もあり得るのでpushだけして続ける
    if pairs.len() == 7 {
        output.push(YakuContext::new(
            hand,
            YakuForm::SevenPair(SevenPair::new(&pairs)),
            agari_tile,
            tsumo,
            yaku_flags,
        ));
    }

    // 副露のvalidationと変換
    let mut blocks: Vec<Block> = Vec::new();
    for fuuro in &ctx.fuuro {
        if !fuuro.valid() {
            return output;
        }
        let Fuuro(ft, v) = fuuro;
        match ft {
            FuuroType::Chi => {
                let mut tile_v: Vec<Tile> = v.iter().map(|TileWithDora(t, _)| *t).collect();
                tile_v.sort();
                blocks.push(Block(BlockType::Chi, tile_v[0]));
            }
            FuuroType::Pon => {
                blocks.push(Block(BlockType::Pon, v[0].0));
            }
            FuuroType::Ankan => {
                blocks.push(Block(BlockType::Ankan, v[0].0));
            }
            FuuroType::Minkan => {
                blocks.push(Block(BlockType::Minkan, v[0].0));
            }
        }
    }

    // TODO: 4面子1雀頭の判定

    output
}
