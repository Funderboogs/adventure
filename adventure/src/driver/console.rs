use std::error::Error;
use crate::driver::Driver;
use crate::game::{
    Game,
    Progress,
    SceneIdentifier,
    Character,
    State,
    MenuItemIdentifier,
    SceneView,
};
use cursive::{
    views::{
        SelectView,
        TextView,
        LinearLayout,
        Button,
        ScrollView,
        Dialog,
        DummyView,
        Panel,
    },
    Cursive, CursiveExt,
};
use std::collections::{HashMap,HashSet};
pub struct ConsoleDriver<'game> {
    pub progress: Progress,
    sui: Cursive,
    game: &'game Game,
}

impl<'game> ConsoleDriver<'game> {
    pub fn new(game: &'game Game) -> Self {
        ConsoleDriver {
            progress: Progress {
                scene: SceneIdentifier::empty(),
                character: Character {
                    stats: HashMap::new(),
                    inventory: HashSet::new(),
                    achievements: HashSet::new(),
                    state: State::Playing,
                },
            },
            sui: Cursive::default(),
            game,
        }
    }
    fn view(&mut self, view: &SceneView) {
        self.sui.clear();
        self.sui.add_fullscreen_layer(
            DummyView::new(),
        );
        self.sui.add_layer(
            LinearLayout::vertical()
            .child(
                Panel::new(
                    ScrollView::new(
                        TextView::new(view.description.clone()),
                    ),
                ),
            )
            .child(
                DummyView::new(),
            )
            .child(
                Button::new("OK", |s| {
                    s.quit();
                }),
            ),
        );
        self.sui.run();
        self.sui.pop_layer();
        self.sui.pop_layer();
    }
    fn get_choice(&mut self, view: &SceneView) -> Option<MenuItemIdentifier> {
        let mut select = SelectView::new();
        for (id, description) in &view.menu {
            select.add_item(description, id.clone());
        }
        select.add_item("Quit", MenuItemIdentifier::from_string("__QUIT"));
        select.set_on_submit(|s, choice| {
            if *choice != "__QUIT" {
                s.set_user_data(choice.clone());
            }
            s.quit();
        });
        self.sui.add_fullscreen_layer(
            DummyView::new(),
        );
        self.sui.add_layer(Dialog::around(select));
        self.sui.run();
        self.sui.pop_layer();
        self.sui.pop_layer();
        self.sui.user_data().cloned()
    }
    
}

impl<'game> Driver<'game> for ConsoleDriver<'game> {
    fn drive(&mut self) -> Result<(), Box<dyn Error>> {
        let mut view = SceneView {
            description: String::new(),
            menu: HashMap::new(),
        };
        self.game.initialize(&mut self.progress, &mut view);
        self.sui = Cursive::default();
        loop {
            self.view(&view);
            if let Some(choice) = self.get_choice(&view) {
                if !self.game.choose(&mut self.progress, &choice, &mut view)? {
                  break
                }
            } else {
                break;
            }
        }
        let message = if self.progress.character.state == State::Won {
            "Congratulations! You won!"
        } else if self.progress.character.state == State::Lost {
            "Sorry, you lost."
        } else {
            "Meh, you left."
        }.to_string();
        self.sui.clear();
        self.sui.add_fullscreen_layer(DummyView::new());
        self.sui.add_layer(
            Dialog::around(TextView::new(message))
                .button("OK", |s| {
                    s.quit();
                }),
        );
        self.sui.run();

        
        Ok(())
    }
}

