use std::fs;

mod html_gen;

fn write_to_template(content: Vec<String>) {
    for i in content  {
        fs::write("/home/bourbon/dev/Inkspace/templates/gen_index.html.tera", i)
            .expect("Failed to generate template!");
    }
}

fn main() {
    let content: Vec<String> = html_gen::call_generator();
    write_to_template(content);
} 
