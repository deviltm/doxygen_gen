#![allow(dead_code)]
use encoding::Encoding;
use iced::{
    theme::Scrollable,
    widget::{button, column, container, pick_list, row, scrollable, text},
    Alignment, Length, Renderer, Sandbox,
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
            encoding: Some("utf-8".to_owned()),
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
                    let mut out = self.output_directory.join(file.file_name().unwrap());
                    out.set_extension("docx");
                    if let Err(e) = export_doc(data, out.clone()){
                        println!("{:#?}",out);
                        println!("{:#?}",e);
                    }
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
        let files = self
            .files
            .iter()
            .map(|f| text(f.file_name().unwrap().to_str().unwrap()).into())
            .collect::<Vec<iced::Element<'_, _>>>();
        let scroll_content = column(files).align_items(Alignment::Start);
        let scroll = scrollable(scroll_content)
            .height(Length::Fill)
            .horizontal_scroll(scrollable::Properties::new())
            .vertical_scroll(scrollable::Properties::new());
        let open_column = column![text("Files:"), scroll, open_button]
            .spacing(10)
            .padding(20)
            .width(300)
            .align_items(Alignment::Center);
        let save_dit_text = text(self.output_directory.display());
        let save_column = column![
            text("Encoding:"),
            encodings_list,
            save_dir_button,
            save_dit_text,
            go_button
        ]
        .spacing(10)
        .padding(20)
        .align_items(Alignment::Center);
        let content = row![open_column, save_column];
        container(content).center_y().center_x().padding(10).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
