mod build;

mod shared;

use clap::{App, SubCommand, Arg};
use colorful::Colorful;
use tokio::fs;
use shared::DEFAULT_PKG;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    println!("{}", shared::TITLE.cyan().bold());

    println!("\nFlylang Compiler\n");

    let app = App::new("Flylang Compiler")
        .about("The Flylang Compiler")
        .subcommands(vec![
            SubCommand::with_name("build")
                .about("Build this project")
                .arg(Arg::from_usage("--release 'compile with release config'"))
                .arg(Arg::from_usage("--debug 'compile with debug config'")),
            SubCommand::with_name("add").about("Add new dependency"),
            SubCommand::with_name("new").about("New Project")
        ]);

    let matches = app.get_matches();

    match matches.subcommand() {
        ("build", _subargs) => { build::subcommand().await?; }
        ("new", _) => {
            fs::write("./package.yml", DEFAULT_PKG).await?;
            println!("Ok\n");
        }
        ("add", _) => {
            println!("Ok\n");
        }
        _ => {
            println!("Use `--help` to print help information.")
        }
    }
    Ok(())
}
