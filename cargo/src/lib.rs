//! # Art
//!
//! A library for modeling artistic concepts.
///Adds one to the number given.
///
/// # Examples
///
/// ```

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}