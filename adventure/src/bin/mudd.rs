use adventure::{Game,TextDriver,ConsoleDriver,Driver};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    game_yaml: Option<PathBuf>,

    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let mut clog = colog::default_builder();
    if args.debug {
        clog.filter(None, log::LevelFilter::Debug);
    } else {
        clog.filter(None, log::LevelFilter::Info);
    }
    clog.init();

    if let Some(game_yaml) = args.game_yaml {
        let file = std::fs::File::open(game_yaml).expect("Failed to open game file");
        let game = Game::from_yaml(file).expect("Failed to load game");
        if args.debug {
            let mut driver = TextDriver::new(&game);
            log::debug!("{:?}", game);
            driver.drive().expect("Failed to run game");
        } else {
            let mut driver = ConsoleDriver::new(&game);
            driver.drive().expect("Failed to run game");
        }
    } else {
        eprintln!("No game file provided");
    }
}
