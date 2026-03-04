use serde::Deserialize;
use clap::Parser;

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
    descendants: Option<u32>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    /// Number of articles to show
    #[arg(value_name="NUMBER", default_value_t=10)]
    number: usize,
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

    let args = Args::parse();

    for (i, id) in top_ids.iter().take(args.number).enumerate() {
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
        println!("  {} comments\n", story.descendants.unwrap_or(0));

    }
    


}
