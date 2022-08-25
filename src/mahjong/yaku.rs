use super::*;

#[derive(Debug)]
pub struct FiveBlock {
    blocks: Vec<Block>,            // すべてのブロック (アガり牌を含む)
    pair_tile: Tile,               // 雀頭の牌
    tsumo: bool,                   // ツモ和了
    bakaze: Tnum,                  // 場風 (東: 1, 南: 2, 西: 3, 北: 4)
    jikaze: Tnum,                  // 自風 (東: 1, 南: 2, 西: 3, 北: 4)
    counts: Counts,                // 面子や牌種別のカウント
    tile_type_cnts: [usize; TYPE], // 牌の種類ごとの個数
    is_open: bool,                 // 鳴きの有無
    iipeikou_count: usize,         // 一盃口, 二盃口用
}

impl FiveBlock {
    pub fn new(blocks: Vec<Block>, tsumo: bool, bakaze: Tnum, jikaze: Tnum) -> Option<Self> {
        if blocks.len() != 5 {
            return None;
        }

        if let Some(pair_tile) = blocks_to_pair_tile(&blocks) {
            let counts = blocks_to_counts(&blocks);
            let tile_type_cnts = blocks_to_tile_type_cnts(&blocks);
            let is_open = blocks_to_is_open(&blocks);
            let iipeikou_count = blocks_to_iipeikou_count(&blocks);
            return Some(Self {
                blocks,
                pair_tile,
                tsumo,
                bakaze,
                jikaze,
                counts,
                tile_type_cnts,
                is_open,
                iipeikou_count,
            });
        }

        None
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

fn blocks_to_counts(blocks: &Vec<Block>) -> Counts {
    let mut cnt = Counts::default();

    for Block(bt, _) in blocks {
        match bt {
            BlockType::Pair => cnt.pair += 1,
            BlockType::Shuntsu => cnt.shuntsu += 1,
            BlockType::Koutsu => cnt.koutsu += 1,
            BlockType::Chi => cnt.chi += 1,
            BlockType::Pon => cnt.pon += 1,
            BlockType::Minkan => cnt.minkan += 1,
            BlockType::Ankan => cnt.ankan += 1,
        }
    }
    cnt.shuntsu_total = cnt.shuntsu + cnt.chi;
    cnt.koutsu_total = cnt.koutsu + cnt.pon + cnt.minkan + cnt.ankan;
    cnt.ankou_total = cnt.koutsu + cnt.ankan;
    cnt.kantsu_total = cnt.minkan + cnt.ankan;

    cnt
}

fn blocks_to_is_open(blocks: &Vec<Block>) -> bool {
    for Block(bt, _) in blocks {
        match bt {
            BlockType::Chi | BlockType::Pon | BlockType::Minkan => {
                return false;
            }
            _ => {}
        }
    }
    true
}

fn blocks_to_pair_tile(blocks: &Vec<Block>) -> Option<Tile> {
    let mut tile: Tile = Tile::default();
    let mut pair_cnt = 0;
    for Block(bt, t) in blocks {
        match bt {
            BlockType::Pair => {
                tile = *t;
                pair_cnt += 1;
            }
            _ => {}
        }
    }
    if pair_cnt == 1 {
        return Some(tile);
    } else {
        return None;
    }
}

fn blocks_to_tile_type_cnts(blocks: &Vec<Block>) -> [usize; TYPE] {
    let mut tile_type_cnts = [0; TYPE];
    for Block(bt, t) in blocks {
        match bt {
            BlockType::Pair => {
                tile_type_cnts[t.0] += 2;
            }
            BlockType::Ankan | BlockType::Minkan => {
                tile_type_cnts[t.0] += 4;
            }
            _ => {
                tile_type_cnts[t.0] += 3;
            }
        }
    }
    tile_type_cnts
}

fn blocks_to_iipeikou_count(blocks: &Vec<Block>) -> usize {
    let mut n = 0;
    let mut shuntsu = TileTable::default();
    for Block(bt, t) in blocks {
        match bt {
            BlockType::Shuntsu => {
                shuntsu[t.0][t.1] += 1;
                if shuntsu[t.0][t.1] == 2 {
                    n += 1;
                }
            }
            _ => {}
        }
    }
    n
}

fn check_yakuhai(blocks: &Vec<Block>) -> TileRow {
    let mut tr = TileRow::default();
    for Block(b, t) in blocks {
        match b {
            BlockType::Koutsu | BlockType::Pon | BlockType::Minkan | BlockType::Ankan => {
                if t.is_honor() {
                    tr[t.1] += 1;
                }
            }
            _ => {}
        }
    }

    tr
}

#[derive(Debug)]
pub struct SevenPair {
    pairs: Vec<Tile>,              // 七対子で使う全ての牌
    tile_type_cnts: [usize; TYPE], // 牌の種類ごとの個数
}

impl SevenPair {
    pub fn new(pairs: &Vec<Tile>) -> Self {
        let mut tile_type_cnts = [0; TYPE];
        for t in pairs {
            tile_type_cnts[t.0] += 2;
        }
        Self {
            pairs: pairs.clone(),
            tile_type_cnts,
        }
    }
}

#[derive(Debug)]
pub enum YakuForm {
    FiveBlock(FiveBlock),
    SevenPair(SevenPair),
    KokushiMusou,
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

#[derive(Debug)]
pub struct YakuContext {
    hand: TileTable,       // 元々の手牌 (鳴き、アガり牌は含まない)
    form: YakuForm,        // 役の形
    agari_tile: Tile,      // アガり牌
    tsumo: bool,           // ツモ和了
    yaku_flags: YakuFlags, // 組み合わせ以外による役 外部から設定を行う
}

impl YakuContext {
    pub fn new(
        hand: TileTable,
        form: YakuForm,
        agari_tile: Tile,
        tsumo: bool,
        yaku_flags: YakuFlags,
    ) -> Self {
        Self {
            hand,
            form,
            agari_tile,
            tsumo,
            yaku_flags,
        }
    }

    // (役一覧, 飜数, 役満倍数)を返却. 役満ではない場合,役満倍率は0, 役一覧に鳴き0飜とドラは含まない
    pub fn calc_yaku(&self) -> (Vec<&'static Yaku>, usize, usize) {
        let mut yaku = vec![];
        for y in YAKU_LIST {
            if (y.func)(self) {
                yaku.push(y)
            }
        }

        let mut yakuman = vec![];
        for &y in &yaku {
            if y.fan_close >= 13 {
                yakuman.push(y);
            }
        }

        if !yakuman.is_empty() {
            let mut m = 0;
            for y in &yakuman {
                m += y.fan_close - 12;
            }
            (yakuman, 0, m) // 役満が含まれている場合,役満以上の役のみを返却
        } else {
            let mut m = 0;
            for y in &yaku {
                m += match &self.form {
                    YakuForm::FiveBlock(b) => {
                        if b.is_open() {
                            y.fan_open
                        } else {
                            y.fan_close
                        }
                    }
                    _ => y.fan_close,
                };
            }
            (yaku, m, 0) // 役満を含んでいない場合
        }
    }

    pub fn calc_fu(&self) -> usize {
        match &self.form {
            YakuForm::FiveBlock(b) => {
                if is_pinfu(self) {
                    return if b.tsumo { 20 } else { 30 };
                }

                // 副底
                let mut fu = 20;

                // 和了り方
                fu += if b.tsumo {
                    2 // ツモ
                } else if !b.is_open() {
                    10 // 門前ロン
                } else {
                    0
                };

                // 面子, 雀頭
                for Block(bt, t) in &b.blocks {
                    match bt {
                        BlockType::Pair => {
                            fu += if t.is_dragon() {
                                2
                            } else if t.is_honor() {
                                if t.1 == b.bakaze || t.1 == b.jikaze {
                                    2
                                } else {
                                    0
                                }
                            } else {
                                0
                            }
                        }
                        BlockType::Koutsu => fu += if t.is_end() { 8 } else { 4 },
                        BlockType::Pon => fu += if t.is_end() { 4 } else { 2 },
                        BlockType::Minkan => fu += if t.is_end() { 16 } else { 8 },
                        BlockType::Ankan => fu += if t.is_end() { 32 } else { 16 },
                        _ => {}
                    }
                }

                // 待ちの形
                let at = &self.agari_tile;
                for Block(bt, t) in &b.blocks {
                    if t.0 != at.0 {
                        continue;
                    }
                    match bt {
                        BlockType::Shuntsu => {
                            // カンチャン待ち,ペンチャン7待ち,ペンチャン3待ち
                            if t.1 + 1 == at.1
                                || (t.1 == at.1 && at.1 == 7)
                                || (t.1 + 2 == at.1 && at.1 == 3)
                            {
                                fu += 2;
                                break;
                            }
                        }
                        BlockType::Koutsu => {} // シャンポン待ち
                        BlockType::Pair => {
                            // タンキ待ち, ノベタン待ち
                            if t.1 == at.1 {
                                fu += 2;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                let fu = (fu + 9) / 10 * 10; // １の位は切り上げ
                if fu == 20 {
                    30 // 例外: 喰いピンフ形
                } else {
                    fu
                }
            }
            YakuForm::SevenPair(_) => 25,
            _ => 0,
        }
    }
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
    yaku!(18, "両立直", is_doubleriichi, 2, 0),
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
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.bakaze == WEA && check_yakuhai(&b.blocks)[WEA] == 1,
        _ => false,
    }
}
fn is_bakaze_s(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.bakaze == WSO && check_yakuhai(&b.blocks)[WSO] == 1,
        _ => false,
    }
}
fn is_bakaze_w(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.bakaze == WWE && check_yakuhai(&b.blocks)[WWE] == 1,
        _ => false,
    }
}
fn is_bakaze_n(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.bakaze == WNO && check_yakuhai(&b.blocks)[WNO] == 1,
        _ => false,
    }
}

// 自風
fn is_jikaze_e(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.jikaze == WEA && check_yakuhai(&b.blocks)[WEA] == 1,
        _ => false,
    }
}
fn is_jikaze_s(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.jikaze == WSO && check_yakuhai(&b.blocks)[WSO] == 1,
        _ => false,
    }
}
fn is_jikaze_w(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.jikaze == WWE && check_yakuhai(&b.blocks)[WWE] == 1,
        _ => false,
    }
}
fn is_jikaze_n(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.jikaze == WNO && check_yakuhai(&b.blocks)[WNO] == 1,
        _ => false,
    }
}

// 白
fn is_haku(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => check_yakuhai(&b.blocks)[DWH] == 1,
        _ => false,
    }
}

// 發
fn is_hatsu(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => check_yakuhai(&b.blocks)[DGR] == 1,
        _ => false,
    }
}

// 中
fn is_chun(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => check_yakuhai(&b.blocks)[DRE] == 1,
        _ => false,
    }
}

// 断么九
fn is_tanyaochuu(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => {
            for Block(bt, t) in &b.blocks {
                match bt {
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
    match &ctx.form {
        YakuForm::FiveBlock(b) => {
            if b.counts.shuntsu != 4 {
                return false;
            }

            let pt = &b.pair_tile;
            if pt.is_honor() && (pt.is_dragon() || pt.1 == b.bakaze || pt.1 == b.jikaze) {
                return false;
            }

            // 上がり牌の両面待ち判定
            let at = &ctx.agari_tile;
            if at.is_honor() {
                return false;
            }
            for Block(bt, t) in &b.blocks {
                match bt {
                    BlockType::Shuntsu => {
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
    match &ctx.form {
        YakuForm::FiveBlock(b) => !b.is_open && b.iipeikou_count == 1,
        _ => false,
    }
}

// 二盃口
fn is_ryanpeikou(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => !b.is_open && b.iipeikou_count == 2,
        _ => false,
    }
}

// 一気通貫
fn is_ikkitsuukan(ctx: &YakuContext) -> bool {
    match &ctx.form {
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
    match &ctx.form {
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
    match &ctx.form {
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
    match &ctx.form {
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
    match &ctx.form {
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
    match &ctx.form {
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
    match &ctx.form {
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
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.counts.koutsu_total == 4,
        _ => false,
    }
}

// 三暗刻
fn is_sanankou(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => {
            if b.counts.ankou_total < 3 {
                return false;
            }

            let mut cnt = 0;
            for Block(bt, t) in &b.blocks {
                if let BlockType::Koutsu = bt {
                    if !ctx.tsumo && ctx.agari_tile == *t {
                        continue;
                    }
                    cnt += 1;
                }
            }

            cnt == 3
        }
        _ => false,
    }
}

// 四暗刻
fn is_suuankou(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => {
            b.counts.ankou_total == 4 && ctx.agari_tile != b.pair_tile && ctx.tsumo
        }
        _ => false,
    }
}

// 四暗刻単騎
fn is_suuankoutanki(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.counts.ankou_total == 4 && ctx.agari_tile == b.pair_tile,
        _ => false,
    }
}

// 三槓子
fn is_sankantsu(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.counts.kantsu_total == 3,
        _ => false,
    }
}

// 四槓子
fn is_suukantsu(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => b.counts.kantsu_total == 4,
        _ => false,
    }
}

// 混一色
fn is_honitsu(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => {
            use std::cmp::min;
            let tile_type_cnts = &b.tile_type_cnts;
            let suit = min(tile_type_cnts[TM], 1)
                + min(tile_type_cnts[TP], 1)
                + min(tile_type_cnts[TS], 1);
            suit == 1 && tile_type_cnts[TZ] > 0
        }
        YakuForm::SevenPair(s) => {
            use std::cmp::min;
            let tile_type_cnts = &s.tile_type_cnts;
            let suit = min(tile_type_cnts[TM], 1)
                + min(tile_type_cnts[TP], 1)
                + min(tile_type_cnts[TS], 1);
            suit == 1 && tile_type_cnts[TZ] > 0
        }
        _ => false,
    }
}

// 清一色
fn is_chinitsu(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => {
            use std::cmp::min;
            let tile_type_cnts = &b.tile_type_cnts;
            let suit = min(tile_type_cnts[TM], 1)
                + min(tile_type_cnts[TP], 1)
                + min(tile_type_cnts[TS], 1);
            suit == 1 && tile_type_cnts[TZ] == 0
        }
        YakuForm::SevenPair(s) => {
            use std::cmp::min;
            let tile_type_cnts = &s.tile_type_cnts;
            let suit = min(tile_type_cnts[TM], 1)
                + min(tile_type_cnts[TP], 1)
                + min(tile_type_cnts[TS], 1);
            suit == 1 && tile_type_cnts[TZ] == 0
        }
        _ => false,
    }
}

// 小三元
fn is_shousangen(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => has_all_dragons(&b) && b.pair_tile.is_dragon(),
        _ => false,
    }
}

// 大三元
fn is_daisangen(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => has_all_dragons(&b) && !b.pair_tile.is_dragon(),
        _ => false,
    }
}

// 三元牌の面子or対子を全て持っているか
fn has_all_dragons(b: &FiveBlock) -> bool {
    let mut dragon_cnt = [false; 3];
    for Block(_, t) in &b.blocks {
        if t.0 == TZ {
            match t.1 {
                DWH => {
                    dragon_cnt[0] = true;
                }
                DGR => {
                    dragon_cnt[1] = true;
                }
                DRE => {
                    dragon_cnt[2] = true;
                }
                _ => {}
            }
        }
    }
    dragon_cnt[0] && dragon_cnt[1] && dragon_cnt[2]
}

// 小四喜
fn is_shousuushii(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => has_all_winds(&b) && b.pair_tile.is_wind(),
        _ => false,
    }
}

// 大四喜
fn is_daisuushii(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => has_all_winds(&b) && !b.pair_tile.is_wind(),
        _ => false,
    }
}

// 風牌の面子or対子を全て持っているか
fn has_all_winds(b: &FiveBlock) -> bool {
    let mut wind_cnt = [false; 4];
    for Block(_, t) in &b.blocks {
        if t.0 == TZ {
            match t.1 {
                WEA => {
                    wind_cnt[0] = true;
                }
                WSO => {
                    wind_cnt[1] = true;
                }
                WWE => {
                    wind_cnt[2] = true;
                }
                WNO => {
                    wind_cnt[3] = true;
                }
                _ => {}
            }
        }
    }
    wind_cnt[0] && wind_cnt[1] && wind_cnt[2] && wind_cnt[3]
}

// 緑一色
fn is_ryuuiisou(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => {
            for Block(bt, t) in &b.blocks {
                match bt {
                    BlockType::Chi | BlockType::Shuntsu => {
                        if t.1 != 2 {
                            return false;
                        }
                    }
                    _ => {
                        if !t.is_green() {
                            return false;
                        }
                    }
                }
            }
            true
        }
        YakuForm::SevenPair(s) => {
            for t in &s.pairs {
                if !t.is_green() {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

// 字一色
fn is_tsuuiisou(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => {
            use std::cmp::min;
            let tile_type_cnts = &b.tile_type_cnts;
            let suit = min(tile_type_cnts[TM], 1)
                + min(tile_type_cnts[TP], 1)
                + min(tile_type_cnts[TS], 1);
            suit == 0
        }
        YakuForm::SevenPair(s) => {
            use std::cmp::min;
            let tile_type_cnts = &s.tile_type_cnts;
            let suit = min(tile_type_cnts[TM], 1)
                + min(tile_type_cnts[TP], 1)
                + min(tile_type_cnts[TS], 1);
            suit == 0
        }
        _ => false,
    }
}

// 九蓮宝燈
fn is_chuurenpoutou(ctx: &YakuContext) -> bool {
    let at = &ctx.agari_tile;
    let cnt = ctx.hand[at.0][at.1];
    is_chuurenpoutou_cmn(ctx) && (cnt == 0 || cnt == 2)
}

// 純正九蓮宝燈
fn is_junseichuurenpoutou(ctx: &YakuContext) -> bool {
    let at = &ctx.agari_tile;
    let cnt = ctx.hand[at.0][at.1];
    is_chuurenpoutou_cmn(ctx) && (cnt == 1 || cnt == 3)
}

// 九蓮宝燈 共通
fn is_chuurenpoutou_cmn(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::FiveBlock(b) => {
            if b.is_open {
                return false;
            }

            let tile_type_cnts = &b.tile_type_cnts;
            let tile_type = if tile_type_cnts[TM] == 14 {
                TM
            } else if tile_type_cnts[TP] == 14 {
                TP
            } else if tile_type_cnts[TS] == 14 {
                TS
            } else {
                return false;
            };

            let mut h = ctx.hand;
            let at = &ctx.agari_tile;
            h[at.0][at.1] += 1;
            if h[tile_type][1] < 3 || h[tile_type][9] < 3 {
                return false;
            }
            for n in 2..=8 {
                if h[tile_type][n] == 0 {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

// 国士無双
fn is_kokushimusou(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::KokushiMusou => {
            let at = &ctx.agari_tile;
            let cnt = ctx.hand[at.0][at.1];
            cnt == 0
        }
        _ => false,
    }
}

// 国士無双十三面待ち
fn is_kokushimusoujuusanmenmachi(ctx: &YakuContext) -> bool {
    match &ctx.form {
        YakuForm::KokushiMusou => {
            let at = &ctx.agari_tile;
            let cnt = ctx.hand[at.0][at.1];
            cnt == 1
        }
        _ => false,
    }
}

// 七対子
fn is_sevenpair(ctx: &YakuContext) -> bool {
    match &ctx.form {
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
fn is_doubleriichi(ctx: &YakuContext) -> bool {
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
