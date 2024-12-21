#[derive(Clone)]
pub struct Character(pub [[bool; 5]; 5]);

pub const BLANK: Character = Character([
    [true, true, true, true, true],
    [true, true, true, true, true],
    [true, true, true, true, true],
    [true, true, true, true, true],
    [true, true, true, true, true],
]);

pub mod number {
    use crate::Character;

    pub const ZERO: Character = Character([
        [true, false, false, false, true],
        [true, false, true, false, true],
        [true, false, true, false, true],
        [true, false, true, false, true],
        [true, false, false, false, true],
    ]);
    pub const ONE: Character = Character([
        [true, false, false, true, true],
        [true, true, false, true, true],
        [true, true, false, true, true],
        [true, true, false, true, true],
        [true, false, false, false, true],
    ]);
    pub const TWO: Character = Character([
        [true, false, false, false, true],
        [true, true, true, false, true],
        [true, false, false, false, true],
        [true, false, true, true, true],
        [true, false, false, false, true],
    ]);
    pub const THREE: Character = Character([
        [true, false, false, false, true],
        [true, true, true, false, true],
        [true, false, false, false, true],
        [true, true, true, false, true],
        [true, false, false, false, true],
    ]);
    pub const FOUR: Character = Character([
        [true, false, true, false, true],
        [true, false, true, false, true],
        [true, false, false, false, true],
        [true, true, true, false, true],
        [true, true, true, false, true],
    ]);
    pub const FIVE: Character = Character([
        [true, false, false, false, true],
        [true, false, true, true, true],
        [true, false, false, false, true],
        [true, true, true, false, true],
        [true, false, false, false, true],
    ]);
    pub const SIX: Character = Character([
        [true, false, false, false, true],
        [true, false, true, true, true],
        [true, false, false, false, true],
        [true, false, true, false, true],
        [true, false, false, false, true],
    ]);
    pub const SEVEN: Character = Character([
        [true, false, false, false, true],
        [true, true, true, false, true],
        [true, true, true, false, true],
        [true, true, true, false, true],
        [true, true, true, false, true],
    ]);
    pub const EIGHT: Character = Character([
        [true, false, false, false, true],
        [true, false, true, false, true],
        [true, false, false, false, true],
        [true, false, true, false, true],
        [true, false, false, false, true],
    ]);
    pub const NINE: Character = Character([
        [true, false, false, false, true],
        [true, false, true, false, true],
        [true, false, false, false, true],
        [true, true, true, false, true],
        [true, false, false, false, true],
    ]);
}
