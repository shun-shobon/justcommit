use clap::Parser;

#[derive(Parser)]
#[command(author, about, version)]
pub struct Args {
    pub openai_token: Option<String>,
}
