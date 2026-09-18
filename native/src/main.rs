mod ai;
mod app;
mod card_widget;
mod cards;
mod engine;
mod i18n;
mod storage;
mod ui;

use gtk::prelude::*;
use libadwaita::prelude::*;


fn main() {
    let _ = libadwaita::init();
    let app = libadwaita::Application::builder()
        .application_id("no.spardame.app")
        .build();
    app.connect_activate(|app| ui::activate(app));
    let _ = app.run();
}
