use iced::Application;
use iced::Settings;
//Module to define all the needed regex
mod regex;
//Header parser module
mod parser;
//Data exporter module
mod exporter;
//module for the iced window
mod main_window;

fn main() -> iced::Result {
    let mut settings = Settings {
        text_multithreading: true,
        ..Default::default()
    };
    settings.window.max_size = Some((500, 400));
    settings.window.min_size = Some((500, 400));
    settings.window.size = (500, 400);
    main_window::MainWindow::run(settings)
}
