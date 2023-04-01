#[allow(dead_code)]
use iced::Sandbox;
//Module to define all the needed regex
mod regex;
//Header parser module
mod parser;
//Data exporter module
mod exporter;
//module for the iced window
mod main_window;

fn main() -> iced::Result {
    main_window::MainWindow::run(iced::Settings::default())
}
