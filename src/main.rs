use anyhow::Result;
use clap::{Parser, Subcommand};
use newgpa::{keyring, logging, security};

#[derive(Debug, Parser)]
#[command(name = "newgpa")]
#[command(about = "Modern privacy assistant for OpenPGP and S/MIME")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print runtime diagnostics without leaking secrets.
    Doctor,
    /// List OpenPGP public keys through GPGME.
    ListKeys,
}

fn main() -> Result<()> {
    logging::init()?;
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Doctor) => {
            security::environment::print_doctor_report()?;
        }
        Some(Command::ListKeys) => {
            for key in keyring::openpgp::list_public_keys()? {
                println!(
                    "{}\t{}\t{}",
                    key.fingerprint,
                    key.user_id,
                    key.expires_at.unwrap_or_else(|| "never".into())
                );
            }
        }
        None => run_default()?,
    }
    Ok(())
}

#[cfg(feature = "gui")]
fn run_default() -> Result<()> {
    newgpa::ui::app::run()
}

#[cfg(not(feature = "gui"))]
fn run_default() -> Result<()> {
    security::environment::print_doctor_report()
}
