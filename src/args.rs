use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, about, version)]
pub struct Args {
    #[clap(long)]
    pub openai_token: Option<String>,
}
