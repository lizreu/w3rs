use core::str::FromStr;
use std::fmt::{self, Display, Formatter};

use w3_sys::{j_player, native, Agent};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Player(Agent<j_player>);

impl Player {
    pub const MAX_PLAYERS: usize = 24;

    pub fn by_id(id: i32) -> Self {
        assert!(
            id >= 0 && id < Self::MAX_PLAYERS as i32,
            "player id out of bounds: {id}",
        );

        let agent = native::player(id).expect("player not found").promote();

        Self(agent)
    }

    pub fn id(&self) -> i32 {
        native::get_player_id(&self.0)
    }

    pub fn name(&self) -> String {
        native::get_player_name(&self.0)
    }

    pub fn set_name(&self, name: &str) {
        let cstring = std::ffi::CString::new(name).expect("invalid player name");
        native::set_player_name(&self.0, &cstring);
    }

    pub fn color(&self) -> PlayerColor {
        let color = native::get_player_color(&self.0);

        PlayerColor::from_id(color)
    }

    pub fn set_color(&self, color: PlayerColor) {
        let color = color as i32;

        native::set_player_color(&self.0, color);
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = self.name();

        write!(f, "Player \"{}\"", name)
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PlayerColor {
    Red,
    Blue,
    Cyan,
    Purple,
    Yellow,
    Orange,
    Green,
    Pink,
    Gray,
    LightBlue,
    DarkGreen,
    Brown,
    Maroon,
    Navy,
    Turquoise,
    Violet,
    Wheat,
    Peach,
    Mint,
    Lavender,
    Coal,
    Snow,
    Emerald,
    Peanut,
}

impl PlayerColor {
    pub fn from_id(id: i32) -> Self {
        match id {
            0 => Self::Red,
            1 => Self::Blue,
            2 => Self::Cyan,
            3 => Self::Purple,
            4 => Self::Yellow,
            5 => Self::Orange,
            6 => Self::Green,
            7 => Self::Pink,
            8 => Self::Gray,
            9 => Self::LightBlue,
            10 => Self::DarkGreen,
            11 => Self::Brown,
            12 => Self::Maroon,
            13 => Self::Navy,
            14 => Self::Turquoise,
            15 => Self::Violet,
            16 => Self::Wheat,
            17 => Self::Peach,
            18 => Self::Mint,
            19 => Self::Lavender,
            20 => Self::Coal,
            21 => Self::Snow,
            22 => Self::Emerald,
            23 => Self::Peanut,
            _ => panic!("invalid player color id: {}", id),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Red => "red",
            Self::Blue => "blue",
            Self::Cyan => "cyan",
            Self::Purple => "purple",
            Self::Yellow => "yellow",
            Self::Orange => "orange",
            Self::Green => "green",
            Self::Pink => "pink",
            Self::Gray => "gray",
            Self::LightBlue => "lightblue",
            Self::DarkGreen => "darkgreen",
            Self::Brown => "brown",
            Self::Maroon => "maroon",
            Self::Navy => "navy",
            Self::Turquoise => "turquoise",
            Self::Violet => "violet",
            Self::Wheat => "wheat",
            Self::Peach => "peach",
            Self::Mint => "mint",
            Self::Lavender => "lavender",
            Self::Coal => "coal",
            Self::Snow => "snow",
            Self::Emerald => "emerald",
            Self::Peanut => "peanut",
        }
    }
}

impl Display for PlayerColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for PlayerColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = match s {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "cyan" | "teal" => Self::Cyan,
            "purple" => Self::Purple,
            "yellow" => Self::Yellow,
            "orange" => Self::Orange,
            "green" => Self::Green,
            "pink" => Self::Pink,
            "gray" | "lg" => Self::Gray,
            "lightblue" | "light blue" | "lb" => Self::LightBlue,
            "darkgreen" | "dark green" | "dg" => Self::DarkGreen,
            "brown" => Self::Brown,
            "maroon" | "red2" => Self::Maroon,
            "navy" | "blue2" => Self::Navy,
            "turquoise" | "cyan2" | "teal2" => Self::Turquoise,
            "violet" | "purple2" => Self::Violet,
            "wheat" | "yellow2" => Self::Wheat,
            "peach" | "orange2" => Self::Peach,
            "mint" | "green2" => Self::Mint,
            "lavender" | "pink2" => Self::Lavender,
            "coal" | "gray2" => Self::Coal,
            "snow" | "white" | "lightblue2" | "lb2" => Self::Snow,
            "emerald" | "darkgreen2" | "dg2" => Self::Emerald,
            "peanut" | "brown2" => Self::Peanut,
            _ => return Err(()),
        };

        Ok(color)
    }
}
