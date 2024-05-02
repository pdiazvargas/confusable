use std::{fs::File, io::Write};

mod level_00;
mod level_01;

#[tokio::main]
async fn main() {
    // Using reqwest library make a call to retrieve the source data from:
    // https://www.unicode.org/Public/security/15.1.0/confusables.txt
    // https://docs.rs/reqwest/latest/reqwest/
    // let raw_data = "# confusables.txt";

    // // Using the table data from level1 parse the table data
    // let confusables = TableData::parse(&raw_data).expect("Failed to load confusable data");

    // let mut data_file = File::create("tables.rs").expect("creation failed");

    // data_file
    //     .write(b"pub const CONFUSABLES: &'static [(char, char)] = &[\n")
    //     .unwrap();

    // for entry in confusables
    //     .iter()
    //     .filter(|row| !row.source_is_ascii() && row.target_is_ascii())
    //     .map(|row| row.format_as_char())
    // {
    //     data_file.write(entry.as_bytes()).unwrap();
    // }

    // data_file.write(b"];\n").unwrap();
    // data_file.flush().unwrap();
}
