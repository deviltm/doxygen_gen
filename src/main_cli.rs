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

#![allow(dead_code)]
use std::{path::PathBuf, sync::{mpsc, Arc, Mutex}};

use clap::{self, command, Parser};
use helpers::process_file;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::{prelude::*, ThreadPoolBuilder};

//Modele for some helper funcs
mod helpers;
//Module to define all the needed regex
mod regex;
//Header parser module
mod parser;
//Data exporter module
mod exporter;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A cli tool to generate docx from doxygen",
    long_about = None,
    override_usage = 
    "\n\
     cli --files \"a.h b.h c.h\" --output out\n\
     cli --files \"a.h b.h c.h\" --output out --encoding windows-1251\n\
     cli --file a.h --file b.h --file c.h -output out"
)]
struct Args {
    #[arg(
        short,
        long,
        help = "Specifies file to process, can be used multiple times",
        required_unless_present("files")
    )]
    file: Vec<String>,
    #[arg(short, long, help = "Specifies output directory")]
    output: String,
    #[arg(
        short,
        long,
        help = "Specifies encoding of the files",
        default_value = "utf-8"
    )]
    encoding: String,
    #[arg(
        short = 'F',
        long,
        help = "Speciefies files to process, a string of files",
        required_unless_present("file")
    )]
    files: Option<String>,
}

fn main() {
    let args = Args::parse();

    let output = PathBuf::from(args.output);
    //check output first so we don't have to parse all the input files if  the directory doesn't exist
    if !output.is_dir() {
        println!("Invalid directory {}",output.display());
        return
    }

    let mut files =  args.file;
    if let Some(f) = args.files{
       files.append(f.split(' ').map(str::to_string).collect::<Vec<String>>().as_mut());
    }
    //An iter of all files
    let files = files.iter().map(PathBuf::from).collect::<Vec<PathBuf>>();
    for f in files.iter(){
        if !f.is_file() && !f.exists(){
            println!("Invalid files {}",f.display());
            return
        }
    }
    let num_files = files.len();

    let encoding = args.encoding.to_lowercase().replace(' ', "-");
    let encoding = encoding::all::encodings().iter().find(|x| x.name() == encoding);
    if encoding.is_none(){
        println!("Invalid encoding");
        return
    }

    let encoding = encoding.unwrap();

    let bar = ProgressBar::new(num_files as u64)
        .with_message(format!("Processing {} files",num_files))
        .with_style(ProgressStyle::default_bar()
            .template(
                "{msg}\n\
                [{bar:20}] {pos}/{len}").unwrap().progress_chars("-> "));

    let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
    let (tx,rx) = mpsc::channel::<()>();
    let tx = Arc::new(Mutex::new(tx));

    pool.spawn(move||{
        files.par_iter().for_each(|f|{
            process_file(f.clone(), &output, encoding.to_owned());
            tx.lock().unwrap().send(()).unwrap();
        });
    });
    
    bar.tick();
    for _ in 0..num_files {
        rx.recv().unwrap();
        bar.inc(1);
    }
}
