mod ui;
mod pipewire;

use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = Application::builder()
    .application_id("org.jack.latency.updater")
    .build();

    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    app.run();
}
