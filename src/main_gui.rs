//#![windows_subsystem = "windows"]

use clap::Parser;
use iced::{Application, Settings};
mod gui;
mod util;
mod wx_util;
mod action;

#[derive(Debug,Default)]
pub struct Flags {
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    
}fn main() -> anyhow::Result<()> {
    let _args = Args::parse();
    let icon = include_bytes!("image/icon.png");
    gui::WxDumpGui::run(Settings {
        default_font: iced::Font::with_name("楷体"),
        // default_font:Some(iced::Font::Default),
        window: iced::window::Settings {
            icon: iced::window::icon::from_file_data(icon, None).ok(),
            ..iced::window::Settings::default()
        },
        flags: Flags {
        },
        ..Settings::default()
    })?;
    Ok(())
}