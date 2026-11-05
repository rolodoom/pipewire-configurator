// Declare the application modules.
mod pipewire;
mod ui;

use gtk::prelude::*;
use gtk::Application;

fn main() {
    // Create the GTK application and assign its unique application ID.
    let app = Application::builder()
        .application_id("org.jack.latency.updater")
        .build();

    // Build and display the application window when the application starts.
    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    // Start the GTK application event loop.
    app.run();
}
