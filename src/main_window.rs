use encoding::Encoding;
use iced::{
    executor, subscription,
    widget::{button, column, container, pick_list, progress_bar, row, scrollable, text},
    Alignment, Application, Command, Length, Subscription, Theme,
};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use rfd::FileDialog;
use std::{
    borrow::Cow,
    ops::RangeInclusive,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::{exporter::export_doc, parser::parse_file};

static BUFFER: Lazy<Arc<Mutex<Vec<PathBuf>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
static FINISHED: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

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
    ProgressChanged(Vec<PathBuf>),
}

fn process_file(r#in: PathBuf, out: &Path, encoding: &dyn Encoding) {
    let data = parse_file(r#in.clone(), encoding.to_owned());
    if data.is_err() {
        println!("Could not parse {}", r#in.display());
    }
    let data = data.unwrap();
    let mut out = out.join(r#in.file_name().unwrap());
    out.set_extension("docx");
    if let Err(e) = export_doc(data, out.clone()) {
        println!("{:#?}", out);
        println!("{:#?}", e);
    }
    BUFFER.lock().unwrap().push(r#in);
}

impl Application for MainWindow {
    type Message = Message;

    fn new(_flags: ()) -> (MainWindow, Command<Message>) {
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
        "Test".to_owned()
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
                //TODO: add empty value checks
                let encoding = encoding::all::encodings()
                    .iter()
                    .find(|x| x.name() == self.encoding.clone().unwrap())
                    .unwrap();

                //Prepare data for multithreading
                let files = self.files.clone();
                // self.files.clear();
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
                    });
                    *FINISHED.lock().unwrap() = true;
                });
            }
            Message::PickList(e) => self.encoding = Some(e),
            Message::ProgressChanged(items) => {
                let mut fin = FINISHED.lock().unwrap();
                if fin.clone() {
                    *fin = false;
                    self.processing = false;
                    println!("Finished processing (update fn)")
                }
                for i in items {
                    self.files
                        .iter()
                        .position(|file| file == &i)
                        .map(|ind| self.files.remove(ind));
                    self.progress.0 += 1;
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let go_button = button("Process").on_press(Message::ProccessButtonClick);
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
            .width(Length::Fill)
            .align_items(Alignment::Center);
        let save_dit_text = text(self.output_directory.display()).width(180);
        let save_column = if self.processing {
            let progress = row![
                progress_bar(
                    RangeInclusive::new(0.0, self.progress.1 as f32),
                    self.progress.0 as f32,
                )
                .width(Length::Fill),
                text(format!("{}/{}", self.progress.0, self.progress.1))
            ].width(180).spacing(10);
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

async fn check_progress() -> (Option<Vec<PathBuf>>, i32) {
    let items = BUFFER.lock().unwrap().clone();
    BUFFER.lock().unwrap().clear();
    (Some(items), 0)
}
