use chatgpt::client::ClientBuilder;
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
    /// Set the chatGPT model, default value is 'gpt-3.5-turbo'
    #[arg(short, long)]
    model: Option<String>,
}

#[tokio::main]
async fn main() -> ChatGPTResult<()> {
    let args = Args::parse();
    let mut builder = ClientBuilder::default().api_key(args.api_key).to_owned();

    if args.prompt {
        let selected_prompt = Select::new(
            "Select system prompt",
            Prompt::iter().map(|p| p.to_string()).collect(),
        )
        .prompt();
        match selected_prompt {
            Ok(system_prompt) => {
                builder = builder
                    .system_message(
                        PROMPTS
                            .get(&system_prompt)
                            .expect("get not key from prompts")
                            .into(),
                    )
                    .to_owned()
            }
            _ => {}
        };
    }

    if args.model.is_some() {
        builder = builder.model(args.model.unwrap()).to_owned();
    }

    let mut client = builder.build()?;
    let mut parent_id = None;
    loop {
        let message = Text::new("Your question:").prompt();
        match message {
            Ok(message) => {
                let mut sp = Spinner::new(Spinners::Dots9, String::new());
                let (answer, answer_id) = client.send_message(message, parent_id).await?;
                parent_id = Some(answer_id);
                sp.stop_with_message(format!("{}: {}", client.model.clone(), answer));
            }
            Err(_) => break,
        }
    }
    Ok(())
}
