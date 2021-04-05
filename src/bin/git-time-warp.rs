use clap::IntoApp;
use git_time_warp::cli::Cli;
use git_time_warp::{Result, VERSION};

fn main() -> Result<()> {
    let app = Cli::into_app().version(VERSION);
    let matches = app.get_matches();
    println!("Hello, world! {:?}", matches);
    Ok(())
}
