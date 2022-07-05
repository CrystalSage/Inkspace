use std::fs;
use std::io::{LineWriter, Write};

mod md_interpreter;
mod html_gen;

fn write_to_template(content: Vec<String>) {
    let file_handle = fs::File::create("/home/bourbon/dev/Inkspace/templates/gen_index.html.tera")
        .expect("File not found!");

    let mut file_handle = LineWriter::new(file_handle);

    for tag in content {
        writeln!(&mut file_handle, "{}", tag)
            .expect("Failed to write to the file!");
    }

    file_handle.flush().unwrap();
}

fn main() {
    html_gen::call_generator();
} 
