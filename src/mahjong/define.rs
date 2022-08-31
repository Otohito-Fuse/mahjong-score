// 型エイリアス
pub type Seat = usize; // 座席
pub type Type = usize; // 牌の種別部分 (萬子,筒子,索子,字牌,花牌)
pub type Tnum = usize; // 牌の数字部分 (1~9)
pub type Dora = usize; // ドラの飜数 (0以上)
pub type Index = usize; // その他Index

pub type Score = i32; // プレイヤーの持ち点
pub type Point = i32; // 打点
pub type Points = (Point, Point, Point); // (ロンの支払い, ツモ・子の支払い, ツモ・親の支払い)

// Number
pub const SEAT: usize = 4; // 座席の数
pub const TYPE: usize = 5; // 牌の種別部分の数 (萬子,筒子,索子,字牌,花牌)
pub const TNUM: usize = 10; // 牌の数字部分の数 (1~9、0は未使用)
pub const TILE: usize = 4; // 同種の牌の数

// Type Index
pub const TM: Type = 0; // Type: Manzu   (萬子)
pub const TP: Type = 1; // Type: Pinzu   (筒子)
pub const TS: Type = 2; // Type: Souzu   (索子)
pub const TZ: Type = 3; // Type: Zihai   (字牌)
pub const TH: Type = 4; // Type: Hanahai (花牌)

// Tnum Index
pub const WEA: Tnum = 1; // Wind:    East  (東)
pub const WSO: Tnum = 2; // Wind:    South (南)
pub const WWE: Tnum = 3; // Wind:    West  (西)
pub const WNO: Tnum = 4; // Wind:    North (北)
pub const DWH: Tnum = 5; // Dragon:  White (白)
pub const DGR: Tnum = 6; // Dragon:  Green (發)
pub const DRE: Tnum = 7; // Dragon:  Red   (中)
pub const ALM: Tnum = 8; // Almighty

pub const FSP: Tnum = 1; // Fapai:  Spring        (春)
pub const FSU: Tnum = 2; // Fapai:  Summer        (夏)
pub const FAU: Tnum = 3; // Fapai:  Autumn        (秋)
pub const FWI: Tnum = 4; // Fapai:  Winter        (冬)
pub const FPL: Tnum = 5; // Fapai:  Plum          (梅)
pub const FOR: Tnum = 6; // Fapai:  Orchid        (蘭)
pub const FCH: Tnum = 7; // Fapai:  Chrysanthemum (菊)
pub const FBA: Tnum = 8; // Fapai:  Bamboo        (竹)
