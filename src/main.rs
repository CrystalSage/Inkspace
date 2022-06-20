use std::fs;

mod html_gen;

fn write_to_template(content: Vec<String>) {
    fs::write("/home/bourbon/dev/Inkspace/templates/gen_index.html.tera", &content[0])
        .expect("Failed to generate template!");
}

fn main() {
    let content: Vec<String> = html_gen::call_generator();
    write_to_template(content);
} 
