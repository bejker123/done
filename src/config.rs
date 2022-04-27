use anyhow::{Context, Result};
use diesel_migrations::{any_pending_migrations, run_pending_migrations};
use libset::config::Config;
use libset::fi;
use std::io::Write;
use std::process::Command;
use gtk4::{CssProvider, StyleContext};
use crate::adw::gdk::Display;

use crate::storage::database::DatabaseConnection;

pub fn set_app() -> Result<()> {
    let config = get_config();
    if !config.is_written() {
        config.write()?;
        Command::new("diesel").args(["migration", "run"]).output()?;
    }
    set_dotenv()?;
    let connection = DatabaseConnection::establish_connection();
    if any_pending_migrations(&connection)? {
        run_pending_migrations(&connection)?;
    }
    Ok(())
}

pub fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("resources/style/ui.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn set_dotenv() -> Result<()> {
    let mut file = std::fs::OpenOptions::new().write(true).open(".env")?;
    let home = dirs::home_dir().with_context(|| "")?;
    let home = home.display();
    let url = format!("DATABASE_URL={home}/.local/share/do/do.db");
    file.write_all(url.as_bytes())?;
    Ok(())
}

fn get_config() -> Config {
    Config::new("do")
        .about("Do is a To Do app for Linux built with Rust and GTK.")
        .author("Eduardo Flores")
        .version("0.1.0")
        .add(fi!("do.db"))
}