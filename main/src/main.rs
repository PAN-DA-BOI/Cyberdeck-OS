use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation, Widget};
use std::cell::RefCell;

fn main() {
    let app = Application::builder()
        .application_id("org.cyberdeck.main")
        .build();

    // List of apps (name, description)
    let apps = vec![
        ("Notepad", "Write documents and code"),
        ("Presentation Maker", "Create PDF slides"),
        ("Presentation App", "Present PDF slides"),
        ("File Viewer", "Open files with apps"),
        ("COMMS", "Send/receive messages and audio"),
        ("Wikipedia", "Offline Wikipedia viewer"),
        ("Research Projects", "Organize project files"),
        ("Snake", "Play the classic game"),
        ("Settings", "Configure your cyberdeck"),
    ];

    app.connect_activate(move |app| {
        // Create the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Cyberdeck OS")
            .default_width(1080)
            .default_height(600)
            .build();

        // Vertical box to hold app rows
        let vbox = GtkBox::new(Orientation::Vertical, 5);

        // Create a button for each app
        let app_buttons: Vec<Button> = apps.iter()
            .map(|(name, desc)| {
                let label = format!("{}: {}", name, desc);
                Button::with_label(&label)
            })
            .collect();

        // Add all buttons to the box
        for button in &app_buttons {
            vbox.pack_start(button, false, false, 0);
        }

        // Track the selected app (starts at 0)
        let selected_app = RefCell::new(0);

        // Highlight the selected app
        let update_selection = |selected: usize| {
            for (i, button) in app_buttons.iter().enumerate() {
                let style = button.get_style_context();
                if i == selected {
                    style.add_class("selected");
                } else {
                    style.remove_class("selected");
                }
            }
        };

        // Initially select the first app
        update_selection(0);

        // Handle key presses for navigation
        window.connect_key_press_event({
            let app_buttons = app_buttons.clone();
            let selected_app = selected_app.clone();
            move |_, key| {
                use gtk::gdk::enums::key;
                use gtk::gdk::EventKey;

                if let Some(keyval) = key.keyval() {
                    match keyval {
                        key::Up => {
                            if *selected_app.borrow() > 0 {
                                *selected_app.borrow_mut() -= 1;
                                update_selection(*selected_app.borrow());
                            }
                        }
                        key::Down => {
                            if *selected_app.borrow() < app_buttons.len() - 1 {
                                *selected_app.borrow_mut() += 1;
                                update_selection(*selected_app.borrow());
                            }
                        }
                        key::Return => {
                            // Launch the selected app
                            let app_index = *selected_app.borrow();
                            let (app_name, _) = apps[app_index];
                            println!("Launching: {}", app_name);
                            // TODO: Replace with actual app launch logic
                        }
                        _ => {}
                    }
                }
                Inhibit(false)
            }
        });

        // Add CSS for the selected app
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_data(
            r#"
            .selected {
                background: #0078d7;
                color: white;
                font-weight: bold;
            }
            button {
                padding: 10px;
                margin: 2px;
                border-radius: 0;
            }
            "#,
        ).unwrap();
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gdk screen"),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Add the box to the window
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}
