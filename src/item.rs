struct item {}
pub enum RuneType {
    Strength,
    Speed,
    Regeneration,
}

pub struct Rune {
    pub rtype: RuneType,
    pub strength: u8,
}
