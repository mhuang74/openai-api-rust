# openai-api-rust
A simple rust client for OpenAI API.

Has a few conveniences, but is mostly at the level of the API itself.

# Installation

```
$ cargo add openai-api-rust
```

# Quickstart

```rust
use openai_api::{api::CompletionArgs, OpenAIClient};

#[tokio::main]
async fn main() {
    let api_token = std::env::var("OPENAI_SK").unwrap();
    let client = OpenAIClient::new(&api_token);
    let prompt = String::from("Once upon a time,");
    println!(
        "{}{}",
        prompt,
        client.complete(prompt.as_str()).await.unwrap()
    );
}
```
# Basic Usage

## Creating a completion

For simple demos and debugging, you can do a completion and use the `Display` instance of a `Completion` object to convert it to a string:

```rust
let response = client.complete("Once upon a time").await?;
println!("{}", response);
```

To configure the prompt more explicitly, you can use the `CompletionArgs` builder:

```rust
let args = openai_api::api::CompletionArgs::builder()
        .prompt("Once upon a time,")
        .engine(Engine::Davinci)
        .max_tokens(20)
        .temperature(0.7)
        .top_p(0.9)
        .stop(vec!["\n".into()]);
let response = args.complete(&client).await?;
println!("Response: {}", response.choices[0].text);
println!("Model used: {}", response.model);
```

See [examples/](./examples)