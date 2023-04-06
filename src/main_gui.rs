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

use iced::Application;
use iced::Settings;

//Modele for some helper funcs
mod helpers;
//Module to define all the needed regex
mod regex;
//Header parser module
mod parser;
//Data exporter module
mod exporter;
//module for the iced window
mod main_window;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    //Locking the windows size bc ui doesn't really adapt properly + locking the size makes tiling WMs(or
    //at least i3) treat it like a floating window
    settings.window.max_size = Some((500, 400));
    settings.window.min_size = Some((500, 400));
    settings.window.size = (500, 400);
    main_window::MainWindow::run(settings)
}
