#![allow(dead_code)]
use std::{borrow::Cow, path::PathBuf};

use encoding::Encoding;
use iced::{
    widget::{button, column, container, pick_list},
    Alignment, Sandbox,
};

pub struct MainWindow {
    files: Vec<PathBuf>,
    encoding: Option<String>,
    output_directory: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFileButtonClick,
    SaveDirectoryButtonClick,
    ProccessButtonClick,
    PickList(String),
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        MainWindow {
            files: Vec::default(),
            encoding: Some("UTF-8".to_owned()),
            output_directory: PathBuf::default(),
        }
    }

    fn title(&self) -> String {
        "Test".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpenFileButtonClick => {},
            Message::SaveDirectoryButtonClick => {},
            Message::ProccessButtonClick => {},
            Message::PickList(e) => self.encoding = Some(e),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let open_button = button("Select files").on_press(Message::OpenFileButtonClick);
        let save_dir_button = button("Save directory").on_press(Message::SaveDirectoryButtonClick);
        let go_button = button("Proccess").on_press(Message::ProccessButtonClick);
        let encodings_list = pick_list(
            encoding::all::encodings()
                .iter()
                .map(|e| e.name().to_owned())
                .filter(|x| x != "error")
                .collect::<Cow<'_, _>>(),
            self.encoding.clone(),
            Message::PickList,
        );

        let content = column![open_button, save_dir_button, go_button, encodings_list]
            .spacing(10)
            .align_items(Alignment::Center);
        container(content).center_y().center_x().padding(10).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
