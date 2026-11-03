use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box as GtkBox,
    Button, ComboBoxText, Grid, Label, Orientation,
};

use crate::pipewire;

const DEFAULT_QUANTUM: &str = "256";
const DEFAULT_RATE: &str = "48000";

const VALID_QUANTUMS: &[&str] = &[
    "16", "32", "48", "64", "128", "144", "256",
"512", "1024", "2048", "4096", "8192",
];

const VALID_RATES: &[&str] = &[
    "44100", "48000", "96000", "192000",
];

// Select the current value in a dropdown.
fn select_value(combo: &ComboBoxText, values: &[&str], current: &str) {
    if let Some(index) = values.iter().position(|v| *v == current) {
        combo.set_active(Some(index as u32));
    } else {
        combo.append_text(current);
        combo.set_active(Some(values.len() as u32));
    }
}

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
    .application(app)
    .title("PipeWire JACK Latency Updater")
    .default_width(420)
    .default_height(180)
    .build();

    let root_box = GtkBox::new(Orientation::Vertical, 12);
    root_box.set_margin_top(12);
    root_box.set_margin_bottom(12);
    root_box.set_margin_start(12);
    root_box.set_margin_end(12);

    let grid = Grid::new();
    grid.set_column_spacing(8);
    grid.set_row_spacing(8);
    grid.set_halign(Align::Center);

    // Quantum dropdown.
    let quantum_label = Label::new(Some("Quantum (samples):"));
    quantum_label.set_halign(Align::Start);

    let quantum_combo = ComboBoxText::new();

    for value in VALID_QUANTUMS {
        quantum_combo.append_text(value);
    }

    // Sample rate dropdown.
    let rate_label = Label::new(Some("Sample Rate (Hz):"));
    rate_label.set_halign(Align::Start);

    let rate_combo = ComboBoxText::new();

    for value in VALID_RATES {
        rate_combo.append_text(value);
    }

    // Read the current PipeWire settings when the application starts.
    let settings = pipewire::get_current_settings();

    match &settings {
        Ok((quantum, rate)) => {
            select_value(&quantum_combo, VALID_QUANTUMS, quantum);
            select_value(&rate_combo, VALID_RATES, rate);
        }
        Err(_) => {
            select_value(&quantum_combo, VALID_QUANTUMS, DEFAULT_QUANTUM);
            select_value(&rate_combo, VALID_RATES, DEFAULT_RATE);
        }
    }

    grid.attach(&quantum_label, 0, 0, 1, 1);
    grid.attach(&quantum_combo, 1, 0, 1, 1);
    grid.attach(&rate_label, 0, 1, 1, 1);
    grid.attach(&rate_combo, 1, 1, 1, 1);

    root_box.append(&grid);

    let info_label = Label::new(None);
    info_label.set_xalign(0.0);
    info_label.set_wrap(true);

    if let Err(error) = &settings {
        info_label.set_text(&format!("Could not read PipeWire settings: {}", error));
    }

    root_box.append(&info_label);

    let button_box = GtkBox::new(Orientation::Horizontal, 8);
    button_box.set_halign(Align::Center);

    // Update button.
    let update_button = Button::with_label("Update Config");

    let quantum_combo_clone = quantum_combo.clone();
    let rate_combo_clone = rate_combo.clone();
    let info_label_clone = info_label.clone();

    update_button.connect_clicked(move |_| {
        let quantum = match quantum_combo_clone.active_text() {
            Some(value) => value.to_string(),
                                  None => return,
        };

        let rate = match rate_combo_clone.active_text() {
            Some(value) => value.to_string(),
                                  None => return,
        };

        match pipewire::apply_settings(&quantum, &rate) {
            Ok(()) => {
                info_label_clone.set_text(
                    &format!("Configuration updated: {}/{}", quantum, rate)
                );
            }
            Err(e) => {
                info_label_clone.set_text(
                    &format!("Failed to update configuration: {}", e)
                );
            }
        }
    });

    button_box.append(&update_button);

    // Quit button.
    let quit_button = Button::with_label("Quit");
    let app_clone = app.clone();

    quit_button.connect_clicked(move |_| {
        app_clone.quit();
    });

    button_box.append(&quit_button);

    root_box.append(&button_box);

    window.set_child(Some(&root_box));
    window.present();
}
