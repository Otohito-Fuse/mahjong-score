use super::*;

pub type TileRow = [usize; TNUM];
pub type TileTable = [TileRow; TYPE]; // 各牌の枚数の二次元配列

#[derive(Debug)]
pub struct HandContext {
    hand: TileTable,       // 元々の手牌(鳴きは含まない)
    fuuro: Vec<Fuuro>,     // 鳴いている面子
    nuki_dora: Vec<Tile>,  // 抜きドラ
    agari_tile: Tile,      // アガり牌
    tsumo: bool,           // ツモ和了
    bakaze: Tnum,          // 場風 (東: 1, 南: 2, 西: 3, 北: 4)
    jikaze: Tnum,          // 自風 (同上)
    yaku_flags: YakuFlags, // 組み合わせ以外による役 外部から設定を行う
}

#[derive(Debug, Clone, Copy)]
pub enum FuuroType {
    Chi,    // チー
    Pon,    // ポン
    Minkan, // 明槓
    Ankan,  // 暗槓
}

// Tileはチーの場合は先頭の牌
#[derive(Debug, Clone, Copy)]
pub struct Fuuro(pub FuuroType, pub Tile);

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
