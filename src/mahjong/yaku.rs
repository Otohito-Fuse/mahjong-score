use super::*;

#[derive(Debug)]
pub struct FiveBlock {
    blocks: Vec<Block>,    // すべてのブロック（アガり牌を含む）
    pair_tile: Tile,       // 雀頭の牌
    agari_tile: Tile,      // アガり牌
    tsumo: bool,           // ツモ和了
    bakaze: Tnum,          // 場風 (東: 1, 南: 2, 西: 3, 北: 4)
    jikaze: Tnum,          // 自風 (東: 1, 南: 2, 西: 3, 北: 4)
    counts: Counts,        // 面子や牌種別のカウント
    is_open: bool,         // 鳴きの有無
    iipeikou_count: usize, // 一盃口, 二盃口用
}

#[derive(Debug)]
pub struct SevenPair {
    pairs: Vec<Tile>, // 七対子で使う全ての牌
    agari_tile: Tile, // アガり牌
}

#[derive(Debug)]
pub struct KokushiMusou {
    tiles: Vec<Tile>, // 手牌
    agari_tile: Tile, // アガり牌
}

#[derive(Debug)]
pub enum YakuForm {
    FiveBlock(FiveBlock),
    SevenPair(SevenPair),
    KokushiMusou(KokushiMusou),
}

#[derive(Debug)]
pub struct YakuContext {
    form: YakuForm,        // 役の形
    nuki_dora: Vec<Tile>,  // 抜きドラ
    tsumo: bool,           // ツモ和了
    yaku_flags: YakuFlags, // 組み合わせ以外による役 外部から設定を行う
}

#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Pair,    // 雀頭
    Shuntsu, // 順子
    Koutsu,  // 刻子
    Chi,     // チー
    Pon,     // ポン
    Minkan,  // 明槓
    Ankan,   // 暗槓
}

// Tileは順子,チーの場合は先頭の牌
#[derive(Debug, Clone, Copy)]
pub struct Block(pub BlockType, pub Tile);

#[derive(Debug, Default)]
struct Counts {
    pair: usize,
    shuntsu: usize,
    koutsu: usize,
    chi: usize,
    pon: usize,
    minkan: usize,
    ankan: usize,
    shuntsu_total: usize, // shuntu + chi
    koutsu_total: usize,  // koutsu + pon + minkan + ankan
    ankou_total: usize,   // koutsu + ankan
    kantsu_total: usize,  // minkan + ankan
}

pub struct Yaku {
    pub id: usize,                      // 雀魂のID
    pub name: &'static str,             // 役名
    pub func: fn(&YakuContext) -> bool, // 役判定関数
    pub fan_close: usize,               // 鳴きなしの飜
    pub fan_open: usize,                // 鳴きありの飜(食い下がり)
}

impl fmt::Debug for Yaku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.name, self.fan_close, self.fan_open)
    }
}

macro_rules! yaku {
    ($id: expr, $n: expr, $f: expr, $c: expr, $o: expr) => {
        Yaku {
            id: $id,
            name: $n,
            func: $f,
            fan_close: $c,
            fan_open: $o,
        }
    };
}

const YAKU_LIST: &[Yaku] = &[
    yaku!(11, "場風 東", is_bakaze_e, 1, 1),
    yaku!(11, "場風 南", is_bakaze_s, 1, 1),
    yaku!(11, "場風 西", is_bakaze_w, 1, 1),
    yaku!(11, "場風 北", is_bakaze_n, 1, 1),
    yaku!(10, "自風 東", is_jikaze_e, 1, 1),
    yaku!(10, "自風 南", is_jikaze_s, 1, 1),
    yaku!(10, "自風 西", is_jikaze_w, 1, 1),
    yaku!(10, "自風 北", is_jikaze_n, 1, 1),
    yaku!(7, "役牌 白", is_haku, 1, 1),
    yaku!(8, "役牌 發", is_hatsu, 1, 1),
    yaku!(9, "役牌 中", is_chun, 1, 1),
    yaku!(12, "断幺九", is_tanyaochuu, 1, 1),
    yaku!(14, "平和", is_pinfu, 1, 0),
    yaku!(13, "一盃口", is_iipeikou, 1, 0),
    yaku!(28, "二盃口", is_ryanpeikou, 3, 0),
    yaku!(16, "一気通貫", is_ikkitsuukan, 2, 1),
    yaku!(17, "三色同順", is_sanshokudoujun, 2, 1),
    yaku!(19, "三色同刻", is_sanshokudoukou, 2, 2),
    yaku!(15, "混全帯幺九", is_chanta, 2, 1),
    yaku!(26, "純全帯幺九", is_junchan, 3, 2),
    yaku!(24, "混老頭", is_honroutou, 2, 2),
    yaku!(41, "清老頭", is_chinroutou, 13, 13),
    yaku!(21, "対々和", is_toitoihou, 2, 2),
    yaku!(22, "三暗刻", is_sanankou, 2, 2),
    yaku!(38, "四暗刻", is_suuankou, 13, 0),
    yaku!(48, "四暗刻単騎", is_suuankoutanki, 14, 0),
    yaku!(20, "三槓子", is_sankantsu, 2, 2),
    yaku!(44, "四槓子", is_suukantsu, 13, 13),
    yaku!(27, "混一色", is_honitsu, 3, 2),
    yaku!(29, "清一色", is_chinitsu, 6, 5),
    yaku!(23, "小三元", is_shousangen, 2, 2),
    yaku!(37, "大三元", is_daisangen, 13, 13),
    yaku!(43, "小四喜", is_shousuushii, 13, 13),
    yaku!(50, "大四喜", is_daisuushii, 14, 14),
    yaku!(40, "緑一色", is_ryuuiisou, 13, 13),
    yaku!(39, "字一色", is_tsuuiisou, 13, 13),
    yaku!(45, "九蓮宝燈", is_chuurenpoutou, 13, 0),
    yaku!(47, "純正九蓮宝燈", is_junseichuurenpoutou, 14, 0),
    // 特殊な組み合わせ
    yaku!(42, "国士無双", is_kokushimusou, 13, 0),
    yaku!(49, "国士無双十三面", is_kokushimusoujuusanmenmachi, 14, 0),
    yaku!(25, "七対子", is_sevenpair, 2, 0),
    // 特殊条件
    yaku!(1, "門前清自摸和", is_menzentsumo, 1, 0),
    yaku!(2, "立直", is_riichi, 1, 0),
    yaku!(18, "両立直", is_dabururiichi, 2, 0),
    yaku!(30, "一発", is_ippatsu, 1, 0),
    yaku!(5, "海底摸月", is_haiteiraoyue, 1, 1),
    yaku!(6, "河底撈魚", is_houteiraoyui, 1, 1),
    yaku!(4, "嶺上開花", is_rinshankaihou, 1, 1),
    yaku!(3, "槍槓", is_chankan, 1, 1),
    yaku!(35, "天和", is_tenhou, 13, 13),
    yaku!(36, "地和", is_tiihou, 13, 13),
];

// 場風
fn is_bakaze_e(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => b.bakaze == WEA && check_yakuhai(&b.blocks)[WEA] == 1,
        _ => false,
    }
}
fn is_bakaze_s(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => b.bakaze == WSO && check_yakuhai(&b.blocks)[WSO] == 1,
        _ => false,
    }
}
fn is_bakaze_w(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => b.bakaze == WWE && check_yakuhai(&b.blocks)[WWE] == 1,
        _ => false,
    }
}
fn is_bakaze_n(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => b.bakaze == WNO && check_yakuhai(&b.blocks)[WNO] == 1,
        _ => false,
    }
}

// 自風
fn is_jikaze_e(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => b.jikaze == WEA && check_yakuhai(&b.blocks)[WEA] == 1,
        _ => false,
    }
}
fn is_jikaze_s(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => b.jikaze == WSO && check_yakuhai(&b.blocks)[WSO] == 1,
        _ => false,
    }
}
fn is_jikaze_w(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => b.jikaze == WWE && check_yakuhai(&b.blocks)[WWE] == 1,
        _ => false,
    }
}
fn is_jikaze_n(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => b.jikaze == WNO && check_yakuhai(&b.blocks)[WNO] == 1,
        _ => false,
    }
}

// 白
fn is_haku(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => check_yakuhai(&b.blocks)[DWH] == 1,
        _ => false,
    }
}

// 發
fn is_hatsu(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => check_yakuhai(&b.blocks)[DGR] == 1,
        _ => false,
    }
}

// 中
fn is_chun(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => check_yakuhai(&b.blocks)[DRE] == 1,
        _ => false,
    }
}

// 断么九
fn is_tanyaochuu(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => {
            for Block(b, t) in &b.blocks {
                match b {
                    BlockType::Chi | BlockType::Shuntsu => {
                        if t.1 <= 1 || t.1 >= 7 {
                            return false;
                        }
                    }
                    _ => {
                        if t.is_end() {
                            return false;
                        }
                    }
                }
            }
        }
        YakuForm::SevenPair(s) => {
            for tile in &s.pairs {
                if tile.is_end() {
                    return false;
                }
            }
        }
        _ => {
            return false;
        }
    }

    true
}

// 平和
fn is_pinfu(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => {
            if b.counts.shuntsu != 4 {
                return false;
            }

            let pt = &b.pair_tile;
            if pt.is_honor() && (pt.is_doragon() || pt.1 == b.bakaze || pt.1 == b.jikaze) {
                return false;
            }

            // 上がり牌の両面待ち判定
            let at = &b.agari_tile;
            if at.is_honor() {
                return false;
            }
            for Block(b, t) in &b.blocks {
                match b {
                    Shuntsu => {
                        if t.0 == at.0 {
                            if t.1 == at.1 && at.1 < 7 {
                                return true;
                            }
                            if t.1 + 2 == at.1 && at.1 > 3 {
                                return true;
                            }
                        }
                    }
                    _ => {}
                }
            }

            false
        }
        _ => false,
    }
}

// 一盃口
fn is_iipeikou(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => !b.is_open && b.iipeikou_count == 1,
        _ => false,
    }
}

// 二盃口
fn is_ryanpeikou(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => !b.is_open && b.iipeikou_count == 2,
        _ => false,
    }
}

// 一気通貫
fn is_ikkitsuukan(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => {
            if b.counts.shuntsu_total < 3 {
                return false;
            }

            let mut has147 = [[false; 3]; 3];
            for Block(b, t) in &b.blocks {
                match b {
                    BlockType::Chi | BlockType::Shuntsu => match t.1 {
                        1 | 4 | 7 => {
                            has147[t.0][t.1 / 3] = true;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            (has147[0][0] && has147[0][1] && has147[0][2])
                || (has147[1][0] && has147[1][1] && has147[1][2])
                || (has147[2][0] && has147[2][1] && has147[2][2])
        }
        _ => false,
    }
}

// 三色同順
fn is_sanshokudoujun(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => {
            if b.counts.shuntsu_total < 3 {
                return false;
            }

            let mut shuntsu_cnt = [[false; 7]; 3];
            for Block(b, t) in &b.blocks {
                match b {
                    BlockType::Shuntsu | BlockType::Chi => {
                        if t.is_suited() {
                            shuntsu_cnt[t.0][t.1 - 1] = true;
                        }
                    }
                    _ => {}
                }
            }

            for i in 0..7 {
                if shuntsu_cnt[0][i] && shuntsu_cnt[1][i] && shuntsu_cnt[2][i] {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

// 三色同刻
fn is_sanshokudoukou(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => {
            if b.counts.shuntsu_total < 3 {
                return false;
            }

            let mut koutsu_cnt = [[false; 9]; 3];
            for Block(b, t) in &b.blocks {
                match b {
                    BlockType::Koutsu | BlockType::Pon | BlockType::Minkan | BlockType::Ankan => {
                        if t.is_suited() {
                            koutsu_cnt[t.0][t.1 - 1] = true;
                        }
                    }
                    _ => {}
                }
            }

            for i in 0..9 {
                if koutsu_cnt[0][i] && koutsu_cnt[1][i] && koutsu_cnt[2][i] {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

// 混全帯幺九
fn is_chanta(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => {
            // 順子がない場合は混老頭になるため false
            if b.counts.shuntsu_total == 0 {
                return false;
            }

            let mut has_honor = false;
            for Block(b, t) in &b.blocks {
                match b {
                    BlockType::Shuntsu | BlockType::Chi => {
                        if t.is_suited() && t.1 >= 2 && t.1 <= 6 {
                            return false;
                        }
                    }
                    _ => {
                        if t.is_simple() {
                            return false;
                        }
                        if t.is_honor() {
                            has_honor = true;
                        }
                    }
                }
            }
            // 純全帯幺九のときには false とするための措置
            has_honor
        }
        // 混老頭になるため七対子の場合も false
        _ => false,
    }
}

// 純全帯幺九
fn is_junchan(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => {
            // 順子がない場合は清老頭になるため false
            if b.counts.shuntsu_total == 0 {
                return false;
            }

            for Block(b, t) in &b.blocks {
                match b {
                    BlockType::Shuntsu | BlockType::Chi => {
                        if !(t.is_suited() && (t.1 == 1 || t.1 == 7)) {
                            return false;
                        }
                    }
                    _ => {
                        if !t.is_terminal() {
                            return false;
                        }
                    }
                }
            }
            true
        }
        // 清老頭になるため七対子の場合も false
        _ => false,
    }
}

// 混老頭
fn is_honroutou(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => {
            let mut has_honor = false;
            let mut has_terminal = false;
            for Block(b, t) in &b.blocks {
                match b {
                    BlockType::Shuntsu | BlockType::Chi => {
                        return false;
                    }
                    _ => {
                        if t.is_simple() {
                            return false;
                        } else if t.is_terminal() {
                            has_terminal = true;
                        } else if t.is_honor() {
                            has_honor = true;
                        }
                    }
                }
            }
            // 字牌か一九牌どちらかの場合は字一色か清老頭になるため false
            has_honor && has_terminal
        }
        YakuForm::SevenPair(s) => {
            let mut has_honor = false;
            let mut has_terminal = false;
            for tile in &s.pairs {
                if tile.is_simple() {
                    return false;
                } else if tile.is_terminal() {
                    has_terminal = true;
                } else if tile.is_honor() {
                    has_honor = true;
                }
            }
            // 字牌か一九牌どちらかの場合は字一色か清老頭になるため false
            has_honor && has_terminal
        }
        _ => false,
    }
}

// 清老頭
fn is_chinroutou(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => {
            for Block(b, t) in &b.blocks {
                match b {
                    BlockType::Shuntsu | BlockType::Chi => {
                        return false;
                    }
                    _ => {
                        if !t.is_terminal() {
                            return false;
                        }
                    }
                }
            }
            true
        }
        YakuForm::SevenPair(s) => {
            for tile in &s.pairs {
                if !tile.is_terminal() {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

// 対々和
fn is_toitoihou(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::FiveBlock(b) => b.counts.koutsu_total == 4,
        _ => false,
    }
}

// 三暗刻
fn is_sanankou(ctx: &YakuContext) -> bool {}

// 四暗刻
fn is_suuankou(ctx: &YakuContext) -> bool {}

// 四暗刻単騎
fn is_suuankoutanki(ctx: &YakuContext) -> bool {}

// 三槓子
fn is_sankantsu(ctx: &YakuContext) -> bool {}

// 四槓子
fn is_suukantsu(ctx: &YakuContext) -> bool {}

// 混一色
fn is_honitsu(ctx: &YakuContext) -> bool {}

// 清一色
fn is_chinitsu(ctx: &YakuContext) -> bool {}

// 小三元
fn is_shousangen(ctx: &YakuContext) -> bool {}

// 大三元
fn is_daisangen(ctx: &YakuContext) -> bool {}

// 小四喜
fn is_shousuushii(ctx: &YakuContext) -> bool {}

// 大四喜
fn is_daisuushii(ctx: &YakuContext) -> bool {}

// 緑一色
fn is_ryuuiisou(ctx: &YakuContext) -> bool {}

// 字一色
fn is_tsuuiisou(ctx: &YakuContext) -> bool {}

// 九蓮宝燈
fn is_chuurenpoutou(ctx: &YakuContext) -> bool {}

// 純正九蓮宝燈
fn is_junseichuurenpoutou(ctx: &YakuContext) -> bool {}

// 国士無双
fn is_kokushimusou(ctx: &YakuContext) -> bool {}

// 国士無双十三面待ち
fn is_kokushimusoujuusanmenmachi(ctx: &YakuContext) -> bool {}

// 七対子
fn is_sevenpair(ctx: &YakuContext) -> bool {
    match ctx.form {
        YakuForm::SevenPair(_) => true,
        _ => false,
    }
}

// 門前自摸
fn is_menzentsumo(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.menzentsumo
}

// リーチ
fn is_riichi(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.riichi && !ctx.yaku_flags.double_riichi
}

// ダブルリーチ
fn is_dabururiichi(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.double_riichi
}

// 一発
fn is_ippatsu(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.ippatsu
}

// 海底撈月
fn is_haiteiraoyue(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.haiteiraoyue
}

// 河底撈魚
fn is_houteiraoyui(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.houteiraoyui
}

// 嶺上開花
fn is_rinshankaihou(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.rinshankaihou
}

// 槍槓
fn is_chankan(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.chankan
}

// 天和
fn is_tenhou(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.tenhou
}

// 地和
fn is_tiihou(ctx: &YakuContext) -> bool {
    ctx.yaku_flags.tiihou
}
