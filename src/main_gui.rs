//#![windows_subsystem = "windows"]

use clap::Parser;
mod action;
mod gui;
mod util;
mod wx_util;

#[derive(Debug, Default)]
pub struct Flags {}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {}
fn main() -> anyhow::Result<()> {
    let _args = Args::parse();
    iced::daemon(
        gui::WxDumpGui::title,
        gui::WxDumpGui::update,
        gui::WxDumpGui::view,
    )
    .subscription(gui::WxDumpGui::subscription)
    .default_font(iced::Font::with_name("楷体"))
    .theme(gui::WxDumpGui::theme)
    .run_with(gui::WxDumpGui::new)
    .map_err(|e| anyhow::anyhow!("{:?}", e))
    // gui::WxDumpGui::run(Settings {
    //     default_font: iced::Font::with_name("楷体"),
    //     // default_font:Some(iced::Font::Default),
    //     window: iced::window::Settings {
    //         icon: iced::window::icon::from_file_data(icon, None).ok(),
    //         ..iced::window::Settings::default()
    //     },
    //     flags: Flags {},
    //     ..Settings::default()
    // })?;
}
