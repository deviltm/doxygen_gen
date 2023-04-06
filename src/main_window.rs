//The GPLv3 License (GPLv3)
//
//Copyright (c) 2023 Ciubix8513
//
//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//any later version.
//
//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <http://www.gnu.org/licenses/>.

use iced::{
    executor,
    futures::channel::mpsc::{channel, Receiver, Sender},
    subscription,
    widget::{button, column, container, pick_list, progress_bar, row, scrollable, text},
    Alignment, Application, Command, Length, Subscription, Theme,
};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use rfd::FileDialog;
use std::{
    borrow::Cow,
    ops::RangeInclusive,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use crate::helpers::process_file;

static CHANEL_SENDER: Lazy<Arc<Mutex<Option<Sender<Option<PathBuf>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));
static CHANEL_RECEIVER: Lazy<Arc<Mutex<Option<Receiver<Option<PathBuf>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub struct MainWindow {
    files: Vec<PathBuf>,
    encoding: Option<String>,
    output_directory: PathBuf,
    processing: bool,
    progress: (i32, i32),
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFileButtonClick,
    PickList(String),
    ProccessButtonClick,
    SaveDirectoryButtonClick,
    ProgressChanged((PathBuf, bool)),
}

impl Application for MainWindow {
    type Message = Message;

    fn new(_flags: ()) -> (MainWindow, Command<Message>) {
        //why 128? idk it's just a nice number
        let (tx, rx) = channel(128);
        *CHANEL_SENDER.lock().unwrap() = Some(tx);
        *CHANEL_RECEIVER.lock().unwrap() = Some(rx);
        (
            MainWindow {
                files: Vec::default(),
                encoding: Some("utf-8".to_owned()),
                output_directory: PathBuf::default(),
                processing: false,
                progress: (0, 0),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        //I use I3 so I have no idea how that looks
        "Doxygen docx exporter".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
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
                let encoding = encoding::all::encodings()
                    .iter()
                    .find(|x| x.name() == self.encoding.clone().unwrap())
                    .unwrap();

                //Prepare data for multithreading
                let files = self.files.clone();
                let output_directory = self.output_directory.clone();
                let pool = rayon::ThreadPoolBuilder::new()
                    //use max num of threads (Add config for that?)
                    .num_threads(0)
                    .build()
                    .unwrap();
                self.processing = true;
                self.progress = (0, self.files.len() as i32);
                pool.spawn(move || {
                    files.par_iter().for_each(|file| {
                        process_file(file.clone(), &output_directory, encoding.to_owned());
                        let _ = CHANEL_SENDER
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .try_send(Some(file.clone()));
                    });
                    let _ = CHANEL_SENDER
                        .lock()
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .try_send(None);
                });
            }
            Message::PickList(e) => self.encoding = Some(e),
            Message::ProgressChanged((item, finished)) => {
                if finished {
                    self.processing = false;
                    println!("Finished processing (update fn)")
                }
                self.files
                    .iter()
                    .position(|file| file == &item)
                    .map(|ind| self.files.remove(ind));
                self.progress.0 += 1;
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut go_button = button("Process");
        let mut open_button = button("Select files");
        let mut save_dir_button = button("Save directory");

        //Disable the buttons when needed
        if !self.processing {
            open_button = open_button.on_press(Message::OpenFileButtonClick);
            save_dir_button = save_dir_button.on_press(Message::SaveDirectoryButtonClick);
            if !self.files.is_empty() && self.output_directory.exists() {
                go_button = go_button.on_press(Message::ProccessButtonClick)
            }
        }

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
            .width(Length::Fill)
            .align_items(Alignment::Center);

        let save_dit_text = text(self.output_directory.display()).width(180);
        //Add in the progress bar if processing 
        let save_column = if self.processing {
            let progress = row![
                progress_bar(
                    RangeInclusive::new(0.0, self.progress.1 as f32),
                    self.progress.0 as f32,
                )
                .width(Length::Fill),
                text(format!("{}/{}", self.progress.0, self.progress.1))
            ]
            .width(180)
            .spacing(10);
            column![
                text("Encoding:"),
                encodings_list,
                save_dir_button,
                save_dit_text,
                go_button,
                progress
            ]
            .spacing(10)
            .padding(20)
            .align_items(Alignment::Center)
        } else {
            column![
                text("Encoding:"),
                encodings_list,
                save_dir_button,
                save_dit_text,
                go_button
            ]
            .spacing(10)
            .padding(20)
            .align_items(Alignment::Center)
        };
        let content = row![open_column, save_column];
        container(content).center_y().center_x().padding(10).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if self.processing {
            return subscription::unfold(0, 0, move |_| check_progress())
                .map(Message::ProgressChanged);
        }
        Subscription::none()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
}

async fn check_progress() -> (Option<(PathBuf, bool)>, i32) {
    let next = CHANEL_RECEIVER.lock().unwrap().as_mut().unwrap().try_next();
    if next.is_err() {
        return (None, 0);
    }
    let next = next.unwrap().unwrap();
    if let Some(path) = next {
        return (Some((path, false)), 0);
    }
    (Some((PathBuf::default(), true)), 0)
}
