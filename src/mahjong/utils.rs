use super::*;

pub fn check_yakuhai(blocks: &Vec<Block>) -> TileRow {
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
