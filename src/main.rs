mod apis;

#[tokio::main]
async fn main() {
    let query = "###S:Generate a puzzle room for the user, create all the most important information to start with such as:

    Base information:
    Difficulty: 0.4
    Genre: High Fantasy
    Overview: An enchanted room
    
    In toml output the following information
    
    [room]
    room_overview # A short overview of the room and puzzle involved
    room_description # A lengthy vividly detailed description of the room
    room_layout # Array with keys for width, length, height, in meters
    
    [puzzle]
    items_available = {item# A array of available items in the room, each having a name and description
    investigation_methods # What methods are available for the players to investigate, be very thorough and give lots of options
    puzzle_details # Details of how the puzzle flows
    expected_solution # The primary solution players are expected to find, be thorough in detailing this with options";
    let output = apis::gpt_query("gpt-3.5-turbo", 1000, query).await;
    println!("{output:?}");
}
