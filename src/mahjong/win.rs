use super::*;

pub fn detect_winning(ctx: &HandContext) -> Vec<YakuContext> {
    let fuuro_cnt = 3 * &ctx.fuuro.len();
    let hand_tiles = &ctx.hand_tiles;
    if fuuro_cnt + hand_tiles.len() != 13 {
        return vec![];
    }

    // HandContext から YakuContext にそのままコピーするものたち
    let bakaze = ctx.bakaze;
    let jikaze = ctx.jikaze;
    let tsumo = ctx.tsumo;
    let TileWithDora(agari_tile, _) = ctx.agari_tile;
    let yaku_flags = ctx.yaku_flags;

    let mut output: Vec<YakuContext> = Vec::new();

    let mut hand = TileTable::default();
    for TileWithDora(tile, _) in hand_tiles {
        hand[tile.0][tile.1] += 1;
    }
    hand[agari_tile.0][agari_tile.1] += 1;
    // hand は 手牌+アガり牌 (鳴いた牌は含まない) の枚数の2次元配列

    {
        // 七対子と国士無双の判定
        let mut pairs: Vec<Tile> = Vec::new();
        let mut kokushi_flg = true;
        for i in 0..3 {
            for j in 1..=9 {
                let n = hand[i][j];
                if n == 2 {
                    pairs.push(Tile(i, j));
                }
                if (j != 1 && j != 9 && n != 0) || ((j == 1 || j == 9) && (n == 0 || n >= 3)) {
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

    // 4面子1雀頭の組み合わせを複数作れる場合は、
    // fuuro_blocks にいくつかブロックを足したもの、を複数作る
    let fuuro_blocks: Vec<Block> = blocks;

    // 4面子1雀頭の判定
    // 手牌の全種類について雀頭になりうるかどうか試す
    for i in 0..4 {
        for j in 1..=9 {
            if hand[i][j] < 2 {
                continue;
            }

            let mut hand_tmp = hand.clone();
            hand_tmp[i][j] -= 2;

            // 残りの牌を面子に分けられるかどうか試す
            if let Some(manzu_mentsu_vec_vec) = get_mentsu(&hand_tmp[TM]) {
                if let Some(pinzu_mentsu_vec_vec) = get_mentsu(&hand_tmp[TP]) {
                    if let Some(souzu_mentsu_vec_vec) = get_mentsu(&hand_tmp[TS]) {
                        if let Some(zihai_koutsu_vec_vec) = get_koutsu(&hand_tmp[TZ]) {
                            // この時だけ4面子1雀頭が成立している

                            for manzu_mentsu_vec in &manzu_mentsu_vec_vec {
                                for pinzu_mentsu_vec in &pinzu_mentsu_vec_vec {
                                    for souzu_mentsu_vec in &souzu_mentsu_vec_vec {
                                        for zihai_koutsu_vec in &zihai_koutsu_vec_vec {
                                            let mut blocks = fuuro_blocks.clone();

                                            // 雀頭
                                            blocks.push(Block(BlockType::Pair, Tile(i, j)));

                                            for manzu_mentsu in manzu_mentsu_vec {
                                                match manzu_mentsu.mentsu_type {
                                                    MentsuType::Shuntsu => {
                                                        blocks.push(Block(
                                                            BlockType::Shuntsu,
                                                            Tile(TM, manzu_mentsu.head),
                                                        ));
                                                    }
                                                    MentsuType::Koutsu => {
                                                        blocks.push(Block(
                                                            BlockType::Koutsu,
                                                            Tile(TM, manzu_mentsu.head),
                                                        ));
                                                    }
                                                }
                                            }
                                            for pinzu_mentsu in pinzu_mentsu_vec {
                                                match pinzu_mentsu.mentsu_type {
                                                    MentsuType::Shuntsu => {
                                                        blocks.push(Block(
                                                            BlockType::Shuntsu,
                                                            Tile(TP, pinzu_mentsu.head),
                                                        ));
                                                    }
                                                    MentsuType::Koutsu => {
                                                        blocks.push(Block(
                                                            BlockType::Koutsu,
                                                            Tile(TP, pinzu_mentsu.head),
                                                        ));
                                                    }
                                                }
                                            }
                                            for souzu_mentsu in souzu_mentsu_vec {
                                                match souzu_mentsu.mentsu_type {
                                                    MentsuType::Shuntsu => {
                                                        blocks.push(Block(
                                                            BlockType::Shuntsu,
                                                            Tile(TS, souzu_mentsu.head),
                                                        ));
                                                    }
                                                    MentsuType::Koutsu => {
                                                        blocks.push(Block(
                                                            BlockType::Koutsu,
                                                            Tile(TS, souzu_mentsu.head),
                                                        ));
                                                    }
                                                }
                                            }
                                            for zihai_koutsu in zihai_koutsu_vec {
                                                match zihai_koutsu.mentsu_type {
                                                    MentsuType::Shuntsu => {}
                                                    MentsuType::Koutsu => {
                                                        blocks.push(Block(
                                                            BlockType::Koutsu,
                                                            Tile(TZ, zihai_koutsu.head),
                                                        ));
                                                    }
                                                }
                                            }

                                            if let Some(fb) =
                                                FiveBlock::new(blocks, tsumo, bakaze, jikaze)
                                            {
                                                output.push(YakuContext::new(
                                                    hand,
                                                    YakuForm::FiveBlock(fb),
                                                    agari_tile,
                                                    tsumo,
                                                    yaku_flags,
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_winning_works_01() {
        let ctx: HandContext = HandContext::new(
            vec![
                TileWithDora(Tile(TM, 1), 0),
                TileWithDora(Tile(TM, 2), 0),
                TileWithDora(Tile(TM, 3), 0),
                TileWithDora(Tile(TM, 5), 0),
                TileWithDora(Tile(TM, 6), 0),
                TileWithDora(Tile(TM, 7), 0),
                TileWithDora(Tile(TP, 6), 0),
                TileWithDora(Tile(TP, 7), 0),
                TileWithDora(Tile(TP, 8), 0),
                TileWithDora(Tile(TS, 6), 0),
                TileWithDora(Tile(TS, 7), 0),
                TileWithDora(Tile(TZ, WWE), 0),
                TileWithDora(Tile(TZ, WWE), 0),
            ],
            vec![],
            TileWithDora(Tile(TS, 8), 0),
            false,
            WEA,
            WNO,
            vec![],
            vec![],
            vec![],
            YakuFlags::default(),
        );
        assert_eq!(detect_winning(&ctx).len(), 1);
    }

    #[test]
    fn detect_winning_works_02() {
        let ctx: HandContext = HandContext::new(
            vec![
                TileWithDora(Tile(TM, 1), 0),
                TileWithDora(Tile(TM, 2), 0),
                TileWithDora(Tile(TM, 3), 0),
                TileWithDora(Tile(TM, 1), 0),
                TileWithDora(Tile(TM, 2), 0),
                TileWithDora(Tile(TM, 3), 0),
                TileWithDora(Tile(TM, 1), 0),
                TileWithDora(Tile(TM, 2), 0),
                TileWithDora(Tile(TM, 3), 0),
                TileWithDora(Tile(TS, 6), 0),
                TileWithDora(Tile(TS, 7), 0),
                TileWithDora(Tile(TZ, WWE), 0),
                TileWithDora(Tile(TZ, WWE), 0),
            ],
            vec![],
            TileWithDora(Tile(TS, 8), 0),
            false,
            WEA,
            WNO,
            vec![],
            vec![],
            vec![],
            YakuFlags::default(),
        );
        assert_eq!(detect_winning(&ctx).len(), 2);
    }
}
