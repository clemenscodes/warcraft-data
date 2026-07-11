//! [`PlayerColor`]: a player's in-game color, with its hex code and RGBA.

use serde::{Deserialize, Serialize};

use warcraft_primitives::Byte;

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum PlayerColor {
    #[default]
    Red = 0,
    Blue = 1,
    Teal = 2,
    Purple = 3,
    Yellow = 4,
    Orange = 5,
    Green = 6,
    Pink = 7,
    Gray = 8,
    LightBlue = 9,
    DarkGreen = 10,
    Brown = 11,
    Maroon = 12,
    Navy = 13,
    Turquoise = 14,
    Violet = 15,
    Wheat = 16,
    Peach = 17,
    Mint = 18,
    Lavender = 19,
    Coal = 20,
    Snow = 21,
    Emerald = 22,
    Peanut = 23,
}

impl PlayerColor {
    pub fn color_code(&self) -> &'static str {
        match self {
            PlayerColor::Red => "cffff0303",
            PlayerColor::Blue => "cff0042ff",
            PlayerColor::Teal => "cff1be7ba",
            PlayerColor::Purple => "cff550081",
            PlayerColor::Yellow => "cfffefc00",
            PlayerColor::Orange => "cfffe890d",
            PlayerColor::Green => "cff21bf00",
            PlayerColor::Pink => "cffe45caf",
            PlayerColor::Gray => "cff939596",
            PlayerColor::LightBlue => "cff7ebff1",
            PlayerColor::DarkGreen => "cff106247",
            PlayerColor::Brown => "cff4f2b05",
            PlayerColor::Maroon => "cff9c0000",
            PlayerColor::Navy => "cff0000c3",
            PlayerColor::Turquoise => "cff00ebff",
            PlayerColor::Violet => "cffbd00ff",
            PlayerColor::Wheat => "cffecce87",
            PlayerColor::Peach => "cfff7a58b",
            PlayerColor::Mint => "cffbfff81",
            PlayerColor::Lavender => "cffdbb8eb",
            PlayerColor::Coal => "cff4f5055",
            PlayerColor::Snow => "cffecf0ff",
            PlayerColor::Emerald => "cff00781e",
            PlayerColor::Peanut => "cffa56f34",
        }
    }

    pub fn rgba(&self) -> [f32; 4] {
        let code = self.color_code();
        let hex = &code[3..9];
        let red = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let green = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let blue = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

        [
            f32::from(red) / 255.0,
            f32::from(green) / 255.0,
            f32::from(blue) / 255.0,
            1.0,
        ]
    }
}

impl From<Byte> for PlayerColor {
    fn from(value: Byte) -> Self {
        use PlayerColor::*;

        match value.get_byte() {
            0 => Red,
            1 => Blue,
            2 => Teal,
            3 => Purple,
            4 => Yellow,
            5 => Orange,
            6 => Green,
            7 => Pink,
            8 => Gray,
            9 => LightBlue,
            10 => DarkGreen,
            11 => Brown,
            12 => Maroon,
            13 => Navy,
            14 => Turquoise,
            15 => Violet,
            16 => Wheat,
            17 => Peach,
            18 => Mint,
            19 => Lavender,
            20 => Coal,
            21 => Snow,
            22 => Emerald,
            23 => Peanut,
            _ => Self::default(),
        }
    }
}

// DDD role: immutable, equality-by-value → Value Object.
impl ddd::Layered for PlayerColor {
    type Layer = ddd::DomainLayer;
}
impl ddd::ValueObject for PlayerColor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_color_rgba_alpha_is_always_one() {
        let colors = [
            PlayerColor::Red,
            PlayerColor::Blue,
            PlayerColor::Teal,
            PlayerColor::Peanut,
        ];
        for color in colors {
            let rgba = color.rgba();
            assert_eq!(rgba[3], 1.0, "alpha should be 1.0 for {color:?}");
        }
    }

    #[test]
    fn player_color_rgba_channels_are_zero_to_one() {
        for channel in PlayerColor::Red.rgba() {
            assert!((0.0..=1.0).contains(&channel));
        }
    }

    #[test]
    fn player_color_from_byte_round_trips_known_values() {
        assert!(matches!(
            PlayerColor::from(Byte::from(0u8)),
            PlayerColor::Red
        ));
        assert!(matches!(
            PlayerColor::from(Byte::from(1u8)),
            PlayerColor::Blue
        ));
        assert!(matches!(
            PlayerColor::from(Byte::from(23u8)),
            PlayerColor::Peanut
        ));
    }
}
