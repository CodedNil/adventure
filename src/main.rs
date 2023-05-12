mod apis;

#[tokio::main]
async fn main() {
    let query = "###S:Generate a puzzle room for the user, create all the most important information to start with such as:

Difficulty Medium, Genre High Fantasy

In a toml output with each key:
Overview: Room Overview, Room Description, Room Layout & Details
Puzzle Details: Items Available, Methods Of Investigation, Puzzle Overview, Expected Solution";
    let output = apis::gpt_info_query("gpt-3.5-turbo", 1000, query).await;
    println!("{output:?}");
}
