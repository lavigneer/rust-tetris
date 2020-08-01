use gtk::prelude::*;

pub struct ScoreBoard {
    pub component: gtk::Box,
}

impl ScoreBoard {
    pub fn new() -> ScoreBoard {
        let component = gtk::Box::new(gtk::Orientation::Vertical, 0);
        component.set_size_request(400, -1);

        let header = gtk::HeaderBar::new();
        let header_title = gtk::Label::with_mnemonic(Some("Score Board"));
        header.set_custom_title(Some(&header_title));
        component.add(&header);

        // let title_label = gtk::Label::with_mnemonic(Some("Score Board"));

        ScoreBoard { component }
    }
}
