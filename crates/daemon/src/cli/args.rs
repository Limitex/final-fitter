use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ffit-daemon")]
#[command(about = "ffit daemon process")]
pub struct Args {
    /// Run in foreground (don't daemonize)
    #[arg(short, long)]
    pub foreground: bool,
}
