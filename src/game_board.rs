use gtk::prelude::*;

pub struct GameBoard {
    component: gtk::Box,
}

impl GameBoard {
    pub fn new() -> GameBoard {
        let component = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let header = gtk::HeaderBar::new();
        let header_title = gtk::Label::with_mnemonic(Some("Game"));
        header.set_custom_title(Some(&header_title));
        component.add(&header);

        GameBoard { component }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.component
    }
}
