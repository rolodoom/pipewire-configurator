// Declare the application modules.
mod pipewire;
mod ui;

use gtk::Application;
use gtk::prelude::*;

// Declare constants
const APP_ID: &str = "io.github.rolodoom.pipewire-configurator";

fn main() {
    // Create the GTK application and assign its unique application ID.
    let app = Application::builder().application_id(APP_ID).build();

    // Build and display the application window when the application starts.
    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    // Start the GTK application event loop.
    app.run();
}
