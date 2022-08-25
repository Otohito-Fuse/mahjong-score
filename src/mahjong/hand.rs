use super::*;

pub type TileRow = [usize; TNUM];
pub type TileTable = [TileRow; TYPE]; // 各牌の枚数の二次元配列

#[derive(Debug)]
pub struct HandContext {
    pub hand_tiles: Vec<TileWithDora>, // 元々の手牌 (鳴き、アガり牌は含まない)
    pub fuuro: Vec<Fuuro>,             // 鳴いている面子
    pub agari_tile: TileWithDora,      // アガり牌
    pub tsumo: bool,                   // ツモ和了
    pub bakaze: Tnum,                  // 場風 (東: 1, 南: 2, 西: 3, 北: 4)
    pub jikaze: Tnum,                  // 自風 (同上)
    pub dora: Vec<Tile>,               // ドラ
    pub ura_dora: Vec<Tile>,           // 裏ドラ
    pub nuki_dora: Vec<TileWithDora>,  // 抜きドラ
    pub yaku_flags: YakuFlags,         // 組み合わせ以外による役 外部から設定を行う
}

impl HandContext {
    pub fn new(
        hand_tiles: Vec<TileWithDora>,
        fuuro: Vec<Fuuro>,
        agari_tile: TileWithDora,
        tsumo: bool,
        bakaze: Tnum,
        jikaze: Tnum,
        dora: Vec<Tile>,
        ura_dora: Vec<Tile>,
        nuki_dora: Vec<TileWithDora>,
        yaku_flags: YakuFlags,
    ) -> Self {
        Self {
            hand_tiles,
            fuuro,
            agari_tile,
            tsumo,
            bakaze,
            jikaze,
            dora,
            ura_dora,
            nuki_dora,
            yaku_flags,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FuuroType {
    Chi,    // チー
    Pon,    // ポン
    Minkan, // 明槓
    Ankan,  // 暗槓
}

#[derive(Debug, Clone)]
pub struct Fuuro(pub FuuroType, pub Vec<TileWithDora>);

impl Fuuro {
    pub fn valid(&self) -> bool {
        let Fuuro(ft, v) = &self;
        match ft {
            FuuroType::Chi => {
                if v.len() != 3 {
                    return false;
                }
                if v[0].0 .0 != v[1].0 .0 || v[1].0 .0 != v[2].0 .0 {
                    return false;
                }
                if v[0].0 .0 != TM && v[0].0 .0 != TP && v[0].0 .0 != TS {
                    return false;
                }
                let mut num_v: Vec<usize> =
                    v.iter().map(|TileWithDora(Tile(_, n), _)| *n).collect();
                num_v.sort();
                if num_v[0] + 1 != num_v[1] || num_v[1] + 1 != num_v[2] {
                    return false;
                }
                true
            }
            FuuroType::Pon => {
                if v.len() != 3 {
                    return false;
                }
                if v[0].0 != v[1].0 || v[1].0 != v[2].0 {
                    return false;
                }
                true
            }
            _ => {
                if v.len() != 4 {
                    return false;
                }
                if v[0].0 != v[1].0 || v[1].0 != v[2].0 || v[2].0 != v[3].0 {
                    return false;
                }
                true
            }
        }
    }
}

// 特殊形&特殊条件の役
#[derive(Debug, Default, Clone, Copy)]
pub struct YakuFlags {
    pub menzentsumo: bool,
    pub riichi: bool,
    pub double_riichi: bool,
    pub ippatsu: bool,
    pub haiteiraoyue: bool,
    pub houteiraoyui: bool,
    pub rinshankaihou: bool,
    pub chankan: bool,
    pub tenhou: bool,
    pub tiihou: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuuro_validation_chi_01() {
        let tile1 = TileWithDora(Tile(TM, 1), 0);
        let tile2 = TileWithDora(Tile(TM, 2), 0);
        let tile3 = TileWithDora(Tile(TM, 3), 0);
        let fuuro = Fuuro(FuuroType::Chi, vec![tile1, tile2, tile3]);
        assert!(fuuro.valid());
    }

    #[test]
    fn fuuro_validation_chi_02() {
        let tile1 = TileWithDora(Tile(TM, 1), 0);
        let tile2 = TileWithDora(Tile(TM, 3), 0);
        let tile3 = TileWithDora(Tile(TM, 2), 0);
        let fuuro = Fuuro(FuuroType::Chi, vec![tile1, tile2, tile3]);
        assert!(fuuro.valid());
    }

    #[test]
    fn fuuro_validation_chi_03() {
        let tile1 = TileWithDora(Tile(TM, 3), 0);
        let tile2 = TileWithDora(Tile(TM, 2), 0);
        let tile3 = TileWithDora(Tile(TM, 1), 0);
        let fuuro = Fuuro(FuuroType::Chi, vec![tile1, tile2, tile3]);
        assert!(fuuro.valid());
    }

    #[test]
    fn fuuro_validation_chi_04() {
        let tile1 = TileWithDora(Tile(TM, 1), 0);
        let tile2 = TileWithDora(Tile(TP, 3), 0);
        let tile3 = TileWithDora(Tile(TM, 2), 0);
        let fuuro = Fuuro(FuuroType::Chi, vec![tile1, tile2, tile3]);
        assert!(!fuuro.valid());
    }

    #[test]
    fn fuuro_validation_chi_05() {
        let tile1 = TileWithDora(Tile(TM, 1), 0);
        let tile2 = TileWithDora(Tile(TM, 4), 0);
        let tile3 = TileWithDora(Tile(TM, 2), 0);
        let fuuro = Fuuro(FuuroType::Chi, vec![tile1, tile2, tile3]);
        assert!(!fuuro.valid());
    }

    #[test]
    fn fuuro_validation_chi_06() {
        let tile1 = TileWithDora(Tile(TM, 1), 0);
        let tile2 = TileWithDora(Tile(TM, 2), 0);
        let tile3 = TileWithDora(Tile(TM, 3), 0);
        let tile4 = TileWithDora(Tile(TM, 4), 0);
        let fuuro = Fuuro(FuuroType::Chi, vec![tile1, tile2, tile3, tile4]);
        assert!(!fuuro.valid());
    }

    #[test]
    fn fuuro_validation_chi_07() {
        let tile1 = TileWithDora(Tile(TS, 8), 0);
        let tile2 = TileWithDora(Tile(TS, 6), 0);
        let tile3 = TileWithDora(Tile(TS, 7), 0);
        let fuuro = Fuuro(FuuroType::Chi, vec![tile1, tile2, tile3]);
        assert!(fuuro.valid());
    }

    #[test]
    fn fuuro_validation_chi_08() {
        let tile1 = TileWithDora(Tile(TZ, 1), 0);
        let tile2 = TileWithDora(Tile(TZ, 2), 0);
        let tile3 = TileWithDora(Tile(TZ, 3), 0);
        let fuuro = Fuuro(FuuroType::Chi, vec![tile1, tile2, tile3]);
        assert!(!fuuro.valid());
    }

    #[test]
    fn fuuro_validation_pon_01() {
        let tile1 = TileWithDora(Tile(TS, 8), 0);
        let tile2 = TileWithDora(Tile(TS, 8), 0);
        let tile3 = TileWithDora(Tile(TS, 8), 0);
        let fuuro = Fuuro(FuuroType::Pon, vec![tile1, tile2, tile3]);
        assert!(fuuro.valid());
    }

    #[test]
    fn fuuro_validation_pon_02() {
        let tile1 = TileWithDora(Tile(TZ, WEA), 0);
        let tile2 = TileWithDora(Tile(TZ, WEA), 1);
        let tile3 = TileWithDora(Tile(TZ, WEA), 2);
        let fuuro = Fuuro(FuuroType::Pon, vec![tile1, tile2, tile3]);
        assert!(fuuro.valid());
    }

    #[test]
    fn fuuro_validation_pon_03() {
        let tile1 = TileWithDora(Tile(TS, 8), 0);
        let tile2 = TileWithDora(Tile(TS, 7), 0);
        let tile3 = TileWithDora(Tile(TS, 6), 0);
        let fuuro = Fuuro(FuuroType::Pon, vec![tile1, tile2, tile3]);
        assert!(!fuuro.valid());
    }
}
