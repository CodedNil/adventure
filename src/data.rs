use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Clone)]
pub struct GameData {
    pub setting: Setting,
    pub player_characters: Vec<PlayerCharacter>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Setting {
    pub name: String,
    pub background: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerCharacter {
    pub name: String,
    pub description: String,
    pub status: String,
    pub items: Vec<Item>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub slot: String,
    pub description: String,
    pub image: String,
}

pub fn load() -> GameData {
    let mut file = File::open("data.toml").expect("Unable to open the data.toml file");
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)
        .expect("Unable to read the data.toml file");

    toml::from_str(&toml_str).unwrap()
}
