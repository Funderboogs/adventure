use std::error::Error;
use crate::driver::Driver;
use crate::game::{
    Game,
    progress::Progress,
    State,
    scene::{
        MenuItemIdentifier,
        SceneView,
    },
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
        BoxedView,
    },
    style::{
        Style,
        Effect,
    },
    Cursive, CursiveExt,
};
use std::collections::HashMap;
pub struct ConsoleDriver<'game> {
    pub progress: Progress,
    sui: Cursive,
    game: &'game Game,
}

impl<'game> ConsoleDriver<'game> {
    pub fn new(game: &'game Game) -> Self {
        ConsoleDriver {
            progress: Progress::new(game),
            sui: Cursive::default(),
            game,
        }
    }
    fn view(&mut self, view: &SceneView) {
        self.sui.clear();
        self.sui.add_fullscreen_layer(
            DummyView::new(),
        );
        let description = match &view.message {
            None => BoxedView::boxed(TextView::new(view.description.clone())),
            Some(message) => BoxedView::boxed(
                LinearLayout::vertical()
                .child(
                    TextView::new(message)
                        .style(Style::from(Effect::Bold)),
                )
                .child(
                    DummyView::new(),
                )
                .child(
                    TextView::new(view.description.clone()),
                ),
            ),
        };
        self.sui.add_layer(
            LinearLayout::vertical()
            .child(
                Panel::new(
                    ScrollView::new(description),
                ),
            )
            .child(
                DummyView::new(),
            )
            .child(
                Button::new("OK", |s| {
                    s.quit();
                }),
            )
            .child(
                DummyView::new(),
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
        select.add_item("Quit", MenuItemIdentifier::from("__QUIT".to_string()));
        select.set_on_submit(|s: &mut Cursive, choice: &MenuItemIdentifier| {
            if choice == "__QUIT" {
                s.set_user_data(None::<MenuItemIdentifier>);
            } else {
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
            message: None,
            description: String::new(),
            menu: HashMap::new(),
        };
        self.game.initialize(&mut self.progress, &mut view);
        self.sui = Cursive::default();
        'main: loop {
            self.view(&view);
            if let Some(choice) = self.get_choice(&view) {
                if !self.game.choose(&mut self.progress, &choice, &mut view)? {
                  break 'main;
                }
            } else {
                break 'main;
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

