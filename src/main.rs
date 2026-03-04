use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
    descendants: u32,
}

fn main() {
    println!("Hacker News Stories!\n");

    let client = reqwest::blocking::Client::new();

    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";

    let top_ids: Vec<u64> = client
        .get(url)
        .send()
        .expect("Failed to fetch top stories")
        .json()
        .expect("Failed to parse IDs");

    let mut input = String::new();

    println!("How many stories?");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let trimmed = input.trim();
    let amount: usize = trimmed.parse::<usize>().expect("Error parsing number");

    for (i, id) in top_ids.iter().take(amount).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");

        let story: Story = client
            .get(&url)
            .send()
            .expect("Failed to fetch story")
            .json()
            .expect("Failed to parse story");

        let link = story.url.as_deref().unwrap_or("(no URL)");
        println!("{}. {} ({} points by {})", i + 1, story.title, story.score, story.by);
        println!("  {}", link);
        println!("  {} comments\n", story.descendants);

    }
    


}
