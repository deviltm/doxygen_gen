#![allow(dead_code)]
use encoding::Encoding;
use iced::{
    widget::{button, column, container, pick_list},
    Alignment, Renderer, Sandbox,
};
use rfd::FileDialog;
use std::{borrow::Cow, path::PathBuf};

use crate::{exporter::export_doc, parser::parse_file};

pub struct MainWindow {
    files: Vec<PathBuf>,
    encoding: Option<String>,
    output_directory: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFileButtonClick,
    PickList(String),
    ProccessButtonClick,
    SaveDirectoryButtonClick,
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
            Message::OpenFileButtonClick => {
                let files = FileDialog::new()
                    .add_filter("Headers", &["h", "hpp"])
                    .pick_files();
                if let Some(files) = files {
                    self.files = files;
                }
            }
            Message::SaveDirectoryButtonClick => {
                let dir = FileDialog::new().pick_folder();
                if let Some(dir) = dir {
                    self.output_directory = dir;
                }
            }
            Message::ProccessButtonClick => {
                //TODO: add empty value checks 
                let encoding = encoding::all::encodings()
                    .iter()
                    .find(|x| x.name() == self.encoding.clone().unwrap())
                    .unwrap();
                for file in &self.files {
                    let data = parse_file(file.clone(), encoding.to_owned()).unwrap();
                    let out = self.output_directory.to_str().unwrap().to_owned()
                        + file.to_str().unwrap()
                        + ".h";
                    export_doc(data, out.into()).unwrap();
                }
            }
            Message::PickList(e) => self.encoding = Some(e),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let go_button = button("Proccess").on_press(Message::ProccessButtonClick);
        let open_button = button("Select files").on_press(Message::OpenFileButtonClick);
        let save_dir_button = button("Save directory").on_press(Message::SaveDirectoryButtonClick);
        let encodings_list = pick_list(
            encoding::all::encodings()
                .iter()
                .map(|e| e.name().to_owned())
                .filter(|x| x != "error")
                .collect::<Cow<'_, _>>(),
            self.encoding.clone(),
            Message::PickList,
        );

        let c = column![open_button, save_dir_button, go_button]
            .spacing(10)
            .align_items(Alignment::Center);
        let content = column![c, encodings_list];
        container(content).center_y().center_x().padding(10).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
