use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile(pub Type, pub Tnum, pub Dora); // (type index, number index, dora bonus)

impl Tile {
    // 数牌
    #[inline]
    pub fn is_suited(&self) -> bool {
        (self.0 == TM || self.0 == TP || self.0 == TS) && (self.1 >= 1 && self.1 <= 9)
    }

    // 字牌
    #[inline]
    pub fn is_honor(&self) -> bool {
        self.0 == TZ && self.1 >= WEA && self.1 <= DRE
    }

    // 花牌
    #[inline]
    pub fn is_flower(&self) -> bool {
        self.0 == TH && self.1 >= FSP && self.1 <= FBA
    }

    // オールマイティ
    #[inline]
    pub fn is_almighty(&self) -> bool {
        self.0 == TZ && self.1 == ALM
    }

    // 1,9牌
    #[inline]
    pub fn is_terminal(&self) -> bool {
        self.is_suited() && (self.1 == 1 || self.1 == 9)
    }

    // 么九牌
    #[inline]
    pub fn is_end(&self) -> bool {
        self.is_honor() || (self.is_suited() && (self.1 == 1 || self.1 == 9))
    }

    // 中張牌
    #[inline]
    pub fn is_simple(&self) -> bool {
        self.is_suited() && self.1 >= 2 && self.1 <= 8
    }

    // 風牌
    #[inline]
    pub fn is_wind(&self) -> bool {
        self.0 == TZ && self.1 >= WEA && self.1 <= WNO
    }

    // 三元牌
    #[inline]
    pub fn is_doragon(&self) -> bool {
        self.0 == TZ && self.1 >= DWH && self.1 <= DRE
    }

    pub fn from_symbol(s: &str) -> Self {
        let b = s.as_bytes();
        let n = b[1] - b'0';
        let d = if b.len() < 2 { 0 } else { b[2] - b'0' };
        let t = match b[0] as char {
            'm' => 0,
            'p' => 1,
            's' => 2,
            'z' => 3,
            'h' => 4,
            _ => panic!("invalid Tile type"),
        };
        Self(t, n as usize, d as usize)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            ['m', 'p', 's', 'z', 'h'][self.0 as usize],
            self.1,
            self.2
        )
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0 != other.0 {
            return Some(self.0.cmp(&other.0));
        }

        if self.1 != other.1 {
            return Some(self.1.cmp(&other.1));
        }

        self.2.partial_cmp(&other.2)
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
