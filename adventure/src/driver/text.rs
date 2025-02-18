use std::collections::HashMap;
use std::error::Error;
use crate::driver::Driver;
use crate::game::{
    Game,
    progress::Progress,
    scene::{
        MenuItemIdentifier,
        SceneView,
    },
    State,
};

pub struct TextDriver<'game> {
    pub progress: Progress,
    game: &'game Game,
}

impl<'game> TextDriver<'game> {
    pub fn new(game: &'game Game) -> Self {
        TextDriver {
            progress: Progress::new(game),
            game,
        }
    }
    fn view(&mut self, view: &SceneView) {
        if let Some(message) = &view.message {
            println!("{}\n", message);
        }
        println!("{}", view.description);
    }
    fn get_choice(view: &SceneView) -> Option<MenuItemIdentifier> {
        for (id, description) in &view.menu {
            println!("{}: {}", id.as_str(), description);
        }
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        if choice.is_empty() || choice == "__QUIT" {
            None
        } else if view.menu.contains_key(&MenuItemIdentifier::from(choice)) {
            Some(MenuItemIdentifier::from(choice))
        } else {
            println!("Invalid choice");
            Self::get_choice(view)
        }
    }
    
}

impl<'game> Driver<'game> for TextDriver<'game> {
    fn drive(&mut self) -> Result<(), Box<dyn Error>> {
        let mut view = SceneView {
            message: None,
            description: String::new(),
            menu: HashMap::new(),
        };
        self.game.initialize(&mut self.progress, &mut view);
        loop {
            self.view(&view);
            if let Some(choice) = Self::get_choice(&view) {
                if !self.game.choose(&mut self.progress, &choice, &mut view)? {
                    break;
                }
            } else {
                break;
            }
        }
        if self.progress.character.state == State::Won {
            println!("Congratulations! You won!");
        } else if self.progress.character.state == State::Lost {
            println!("Sorry, you lost.");
        }
        Ok(())
    }
}


