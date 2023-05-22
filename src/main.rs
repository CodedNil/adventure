mod apis;
mod data;
mod ui;

fn main() {
    // Apis to access and change the data
    // Add NPC, species & subspecies (generates name, status, history, items)
    // Add species, name (generates description, photo_description, image, status_towards_players, 1 subspecies)
    // Add subspecies, name (generates description, photo_description, image)

    // Add room (generates description, photo_description, image)

    // Add player character (Users customisation UI with buttons to generate things) (generates items)

    // Add item (generates description, photo_description, image)

    // Flow of the gameplay loop from DM perspective
    // DM at any point has buttons to create factions, variants, animals, etc etc, to be used in future
    // Create a zone, either brief description provided or let the AI generate it
    // AI then generates all the rooms of the zone and joins them up, placing animals and npcs etc where needed, and placing loot and puzzles etc

    // Create game ui with journal to view all the data, and buttons to add new data
    ui::create();
}
