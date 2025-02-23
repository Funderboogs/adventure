use adventure::{Game,TextDriver,ConsoleDriver,Driver};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short='y', long)]
    game_yaml: Option<PathBuf>,

    #[clap(short='d', long)]
    game_dsl: Option<PathBuf>,

    #[clap(short='g', long)]
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
    } else if let Some(game_dsl) = args.game_dsl {
        use tree_sitter::Parser;
        use std::io::read_to_string;
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_adventure::LANGUAGE.into()).expect("Failed to load language");
        let file = std::fs::File::open(game_dsl).expect("Failed to open game file");
        let scode = read_to_string(file).expect("Failed to read game file");
        let code = scode.as_bytes();
        let tree = parser.parse(code, None).expect("Failed to parse game file");
        let mut cursor = tree.walk();
        cursor.goto_first_child();
        println!("{:?}", &cursor.node());
        cursor.goto_first_child();
        while !cursor.node().is_named() {
            cursor.goto_next_sibling();
        }
        println!("{:?}", &cursor.node());
        cursor.goto_first_child();
        while !cursor.node().is_named() {
            cursor.goto_next_sibling();
        }
        println!("The character's name is {}", &cursor.node().utf8_text(code).unwrap());



    } else {
        eprintln!("No game file provided");
    }
}
