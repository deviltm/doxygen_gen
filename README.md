# doxygen_gen
An gui utility that generates docx files based on doxygen, tho it is made for a very specific file format.
But I bet it can easily be modified to suit whatever needs you have.

It contains both a cli and an iced gui

The app uses rayon to spread the load between multiple cores, so it's FAST

# CLI
#### Building
```Bash
cargo build --release --bin doxygen_gen-cli
```
I am not quite sure, but I think you need rustc version > 1.67 
#### Options
```
  -f, --file <FILE>          Specifies file to process, can be used multiple times
  -o, --output <OUTPUT>      Specifies output directory
  -e, --encoding <ENCODING>  Specifies encoding of the files [default: utf-8]
  -F, --files <FILES>        Speciefies files to process, a string of files
  -h, --help                 Print help
  -V, --version              Print version
  ```
#### Examples
```Bash
doxygen_gen-cli --files "a.h b.h c.h" --output out
doxygen_gen-cli --files "a.h b.h" --file c.h --output out
doxygen_gen-cli --files "a.h b.h c.h" --output out --encoding windows-1251
doxygen_gen-cli --file a.h --file b.h --file c.h -output out
```

# GUI
#### Building
```
cargo build --release --bin doxygen_gen-gui
```
##### Usage
1. Select files by clicking the select files button
2. Select saving direcory by pressing the Save direcory button (Optionally select encoding of the files from the encoding dropdown)
3. Press the Process button

