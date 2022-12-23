///! Example that prints out a story from a prompt. Used in the readme.
use openai_api::Client;

fn main() {
    let api_token = std::env::var("OPENAI_API_KEY").unwrap();
    let client = Client::new(&api_token);
    let prompt = String::from("Once upon a time,");
    println!(
        "{}{}",
        prompt,
        client.complete_prompt_sync(prompt.as_str()).unwrap()
    );
}
