use chatgpt::client::{Client, ClientBuilder};
use chatgpt::error::ChatGPTResult;
use chatgpt::prompt::{Prompt, PROMPTS};
use clap::Parser;
use inquire::{Select, Text};
use spinners::{Spinner, Spinners};
use strum::IntoEnumIterator;

/// ChatGPT command line thar support multiple prompts.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Api key of the openai support
    #[arg(short, long)]
    api_key: String,
    /// Customize the prompt
    #[arg(short, long)]
    prompt: bool,
}

#[tokio::main]
async fn main() -> ChatGPTResult<()> {
    let args = Args::parse();
    let mut client = Client::new(args.api_key.clone())?;

    if args.prompt {
        client = build_with_prompt(args.api_key.clone())?;
    }

    let mut parent_id = None;
    loop {
        let message = Text::new("Your question:").prompt();
        match message {
            Ok(message) => {
                let mut sp = Spinner::new(Spinners::Dots9, String::new());
                let (answer, answer_id) = client.send_message(message, parent_id).await?;
                parent_id = Some(answer_id);
                sp.stop_with_message(answer);
            }
            Err(_) => break,
        }
    }
    Ok(())
}

fn build_with_prompt(api_key: String) -> ChatGPTResult<Client> {
    let selected_prompt = Select::new(
        "Select system prompt",
        Prompt::iter().map(|p| p.to_string()).collect(),
    )
        .prompt();
    let client = match selected_prompt {
        Ok(system_prompt) => ClientBuilder::default()
            .api_key(api_key)
            .system_message(PROMPTS.get(&system_prompt).expect("get not key from prompts").into())
            .build()?,
        Err(_) => Client::new(api_key)?
    };
    Ok(client)
}