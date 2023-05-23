use crate::apis;
use serde::Deserialize;
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
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

pub fn load() -> Game {
    let mut file = File::open("data.toml").expect("Unable to open the data.toml file");
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)
        .expect("Unable to read the data.toml file");

    toml::from_str(&toml_str).unwrap()
}

pub async fn create_character() -> PlayerCharacter {
    let mut overview = String::new();
    let mut name = String::new();
    let mut description = String::new();
    let mut status = String::new();
    let mut items = Vec::new();

    let queries = vec![
        "gender",
        "height",
        "bulk",
        "eye colour",
        "hair style and colour",
        "facial hair",
        "skin tone",
        "face description",
        "demeaner",
        "personality",
        "order these skills, strength, agility, intelligence, magical ability, charisma, stamina",
    ];

    println!("Enter characters high level description, species, abilities (e.g dwarf scholar with a keen interest in arcane knowledge and spellcasting):");
    // io::stdin()
    //     .read_line(&mut overview)
    //     .expect("Failed to read line");
    overview = "a male frog humanoid with water magic, friendly and adventurous spirit".to_string();

    let name_suggestions = apis::gpt_query("gpt-3.5-turbo", 100, format!("S:You generate content for an rpg game###U:My character is described as {overview} suggest a few names based on this, make them interesting and unique, the response should be on a single line and concise###A:What would you like your character to be called? May I suggest ").as_str()).await.unwrap();
    println!("What would you like your character to be called? May I suggest {name_suggestions}");
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");
    name = "Aquazul".to_string();

    // println!("Enter character status:");
    // io::stdin()
    //     .read_line(&mut status)
    //     .expect("Failed to read line");

    PlayerCharacter {
        name: name.trim().to_string(),
        description: description.trim().to_string(),
        status: status.trim().to_string(),
        items,
    }
}
