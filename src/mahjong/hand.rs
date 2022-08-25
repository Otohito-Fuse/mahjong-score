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

// 特殊形&特殊条件の役
#[derive(Debug, Default, Clone)]
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
