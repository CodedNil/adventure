use serde::Deserialize;
use std::fs::File;
use std::io::Read;

mod apis;

// #[tokio::main]
// async fn main() {
//     let query = "###S:Generate a d&d type scenario for players to engage in, it should have an overarching story which is loose so players can choose to go in the directions they choose, and it should have many planned potential scenarios and stories

// Base information:
// Difficulty: 0.4
// Genre: High Fantasy

// (This is an example of toml arrays)
// data = [ [\"delta\", \"phi\"], [3.14] ]
// temp_targets = { cpu = 79.5, case = 72.0 }

// [characters]
// player1 = \"Frogar;Wizard with an obsession with fire\"
// player2
// player3

// [story]
// setting # String that briefly describes the overarching setting
// party_origin # How did the party of adventures end up together in this quest
// story # String that briefly describes the story the players will embark on, and why they are involved

// [npcs]
// primary # An array of strings, each is \"name;brief_description\", the main npcs involved in this story
// secondary # Secondary npcs
// tertiary # NPCs that might pop up but might not
// hostiles # Types of hostiles that the players might encounter, list dozens with just the name of the creature and potentially variants like \"goblin;grunt,archer\"

// In toml output the above information with missing details filled";
//     let output = apis::gpt_query("gpt-3.5-turbo", 1000, query).await;
//     println!("{output:?}");
// }

#[derive(Debug, Deserialize)]
struct GameData {
    setting: Setting,
    player_characters: Vec<PlayerCharacter>,
    rooms: Vec<Room>,
    factions: Vec<Faction>,
    npcs: Vec<NPC>,
}

#[derive(Debug, Deserialize)]
struct Setting {
    name: String,
    background: String,
    description_brief: String,
    description_detailed: String,
}

#[derive(Debug, Deserialize)]
struct PlayerCharacter {
    name: String,
    description: String,
    status: String,
    items: Vec<Item>,
}
#[derive(Debug, Deserialize)]
struct NPC {
    name: String,
    faction: String,
    variant: String,
    status: String,
    history: String,
    items: Vec<Item>,
}
#[derive(Debug, Deserialize)]
struct Item {
    name: String,
    slot: String,
    description: String,
    photo_description: String,
    pointer_to_image: String,
}

#[derive(Debug, Deserialize)]
struct Room {
    id: usize,
    linked_rooms: Vec<usize>,
    description: String,
    photo_description: String,
    image: String,
}

#[derive(Debug, Deserialize)]
struct Faction {
    name: String,
    description: String,
    photo_description: String,
    image: String,
    status_towards_players: String,
    variants: Vec<Variant>,
}
#[derive(Debug, Deserialize)]
struct Variant {
    name: String,
    description: String,
    photo_description: String,
    image: String,
}

fn main() {
    let mut file = File::open("data.toml").expect("Unable to open the data.toml file");
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)
        .expect("Unable to read the data.toml file");

    let data: GameData = toml::from_str(&toml_str).unwrap();

    println!("Data: {data:#?}");

    // Apis to access and change the data
    // Add NPC, species & subspecies (generates name, status, history, items)
    // Add species, name (generates description, photo_description, image, status_towards_players, 1 subspecies)
    // Add subspecies, name (generates description, photo_description, image)

    // Add room (generates description, photo_description, image)

    // Add player character (Users customisation UI with buttons to generate things) (generates items)

    // Add item (generates description, photo_description, image)

    // Create game ui with journal to view all the data, and buttons to add new data
}
