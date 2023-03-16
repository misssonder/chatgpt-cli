use chatgpt::client::Client;
use chatgpt::error::ChatGptResult;
use clap::Parser;
use inquire::Text;
use spinners::{Spinner, Spinners};

/// Simple ChatGPT command line
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Api key of the openai support
    #[arg(short, long)]
    api_key: String,
}

#[tokio::main]
async fn main() -> ChatGptResult<()> {
    let args = Args::parse();
    let mut client = Client::new(args.api_key)?;
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
