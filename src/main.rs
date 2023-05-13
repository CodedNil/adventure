mod apis;

#[tokio::main]
async fn main() {
    let query = "###S:Generate a d&d type scenario for players to engage in, it should have an overarching story which is loose so players can choose to go in the directions they choose, and it should have many planned potential scenarios and stories

Base information:
Difficulty: 0.4
Genre: High Fantasy

(This is an example of toml arrays)
data = [ [\"delta\", \"phi\"], [3.14] ]
temp_targets = { cpu = 79.5, case = 72.0 }

[characters]
player1 = \"Frogar;Wizard with an obsession with fire\"
player2
player3

[story]
setting # String that briefly describes the overarching setting
party_origin # How did the party of adventures end up together in this quest
story # String that briefly describes the story the players will embark on, and why they are involved

[npcs]
primary # An array of strings, each is \"name;brief_description\", the main npcs involved in this story
secondary # Secondary npcs
tertiary # NPCs that might pop up but might not
hostiles # Types of hostiles that the players might encounter, list dozens with just the name of the creature and potentially variants like \"goblin;grunt,archer\"

In toml output the above information with missing details filled";
    let output = apis::gpt_query("gpt-3.5-turbo", 1000, query).await;
    println!("{output:?}");
}
