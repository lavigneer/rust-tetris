extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Box, Separator};

mod game_board;
mod score_board;

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Tetris");
        window.set_default_size(1280, 720);

        let container = Box::new(gtk::Orientation::Horizontal, 0);

        let game_board = game_board::GameBoard::new();
        container.pack_start(game_board.widget(), true, true, 0);

        let separator = Separator::new(gtk::Orientation::Vertical);
        container.pack_start(&separator, false, false, 0);

        let score_board = score_board::ScoreBoard::new();
        container.add(&score_board.component);

        window.add(&container);

        window.show_all();
    });

    application.run(&[]);
}
