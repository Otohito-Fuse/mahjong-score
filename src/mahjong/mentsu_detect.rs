use super::*;

pub type NumCntTable = [usize; TNUM];

trait NumCntTableTrait {
    fn cnt(&self) -> usize;
    fn get_min(&self) -> Option<usize>;
}

impl NumCntTableTrait for NumCntTable {
    fn cnt(&self) -> usize {
        let mut cnt = 0;
        for i in 0..10 {
            cnt += self[i];
        }
        cnt
    }
    fn get_min(&self) -> Option<usize> {
        for i in 0..10 {
            if self[i] > 0 {
                return Some(i);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mentsu {
    pub mentsu_type: MentsuType,
    pub head: usize,
}

impl Mentsu {
    fn new(is_koutsu: bool, head: usize) -> Self {
        if is_koutsu {
            Mentsu {
                mentsu_type: MentsuType::Koutsu,
                head,
            }
        } else {
            Mentsu {
                mentsu_type: MentsuType::Shuntsu,
                head,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MentsuType {
    Shuntsu,
    Koutsu,
}

pub fn get_mentsu(table: &NumCntTable) -> Option<Vec<Vec<Mentsu>>> {
    if table.cnt() % 3 != 0 {
        return None;
    }

    if let Some(i) = table.get_min() {
        if table.cnt() == 3 {
            if table[i] == 3 {
                return Some(vec![vec![Mentsu::new(true, i)]]);
            } else if i < 8 && table[i] == 1 && table[i + 1] == 1 && table[i + 2] == 1 {
                return Some(vec![vec![Mentsu::new(false, i)]]);
            } else {
                return None;
            }
        }
        let mut output: Vec<Vec<Mentsu>> = Vec::new();
        if table[i] >= 3 {
            let mut table = table.clone();
            table[i] -= 3;

            if let Some(v) = get_mentsu(&table) {
                for v in &v {
                    let mut v = v.clone();
                    v.push(Mentsu::new(true, i));
                    output.push(v);
                }
            }
        }
        if i < 8 && table[i] >= 1 && table[i + 1] >= 1 && table[i + 2] >= 1 {
            let mut table = table.clone();
            table[i] -= 1;
            table[i + 1] -= 1;
            table[i + 2] -= 1;

            if let Some(v) = get_mentsu(&table) {
                for v in &v {
                    let mut v = v.clone();
                    v.push(Mentsu::new(false, i));
                    output.push(v);
                }
            }
        }
        return Some(output);
    }

    Some(vec![Vec::new()])
}

pub fn get_koutsu(table: &NumCntTable) -> Option<Vec<Vec<Mentsu>>> {
    if table.cnt() % 3 != 0 {
        return None;
    }

    if let Some(i) = table.get_min() {
        if table.cnt() == 3 {
            if table[i] == 3 {
                return Some(vec![vec![Mentsu::new(true, i)]]);
            } else {
                return None;
            }
        }
        let mut output: Vec<Vec<Mentsu>> = Vec::new();
        if table[i] >= 3 {
            let mut table = table.clone();
            table[i] -= 3;

            if let Some(v) = get_mentsu(&table) {
                for v in &v {
                    let mut v = v.clone();
                    v.push(Mentsu::new(true, i));
                    output.push(v);
                }
            }
        }
        return Some(output);
    }

    Some(Vec::new())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_mentsu_01() {
        let table: NumCntTable = [0, 1, 1, 1, 0, 0, 0, 0, 0, 0];
        assert_eq!(get_mentsu(&table).unwrap().len(), 1);
    }

    #[test]
    fn test_get_mentsu_02() {
        let table: NumCntTable = [0, 1, 1, 1, 0, 0, 0, 0, 0, 0];
        assert_eq!(get_mentsu(&table).unwrap()[0].len(), 1);
    }

    #[test]
    fn test_get_mentsu_03() {
        let table: NumCntTable = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(get_mentsu(&table).unwrap().len(), 1);
    }

    #[test]
    fn test_get_mentsu_04() {
        let table: NumCntTable = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(get_mentsu(&table).unwrap()[0].len(), 0);
    }

    #[test]
    fn test_get_mentsu_05() {
        let table: NumCntTable = [0, 1, 1, 1, 0, 1, 1, 1, 0, 0];
        assert_eq!(get_mentsu(&table).unwrap()[0].len(), 2);
    }

    #[test]
    fn test_get_mentsu_06() {
        let table: NumCntTable = [0, 1, 1, 1, 0, 1, 2, 2, 1, 0];
        assert_eq!(get_mentsu(&table).unwrap()[0].len(), 3);
    }

    #[test]
    fn test_get_mentsu_07() {
        let table: NumCntTable = [0, 1, 1, 1, 0, 2, 2, 2, 0, 0];
        assert_eq!(get_mentsu(&table).unwrap()[0].len(), 3);
    }
}
