use std::io::{BufReader, BufRead};
use std::fs::File;

// A token that is produced after lexing.
struct Token {
    // Data of the token
    content: String,
    // Type of Markdown element
    token_type: MDType, 
    // Span of the content: [from, to]
    span: [u8; 2],
}

enum MDType {
    Heading,
    Link,
    Paragraph,
    Break,
}


#[derive(Debug)]
enum HeadingType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug)]
struct Heading {
    level: HeadingType,
    content: String,
}

fn analyze_heading(line: String) -> Heading {
    let mut depth: usize = 0;
    let content_chars: Vec<char> = line.chars().collect();

    // TODO: Maybe use a `Peekable` iterator.
    // FIXME: We currently allow for content immediately after heading markers.
    // i.e. '#Hello' is allowed (But shouldn't be).
    for marker in &content_chars {
        match marker {
            '#' => { depth = depth + 1;}
            _ => {break;}
        }
    }

    let data: String = content_chars[depth..].into_iter().collect();
    let data: String = data.trim_start().to_string();

    Heading {
        level: match depth{
            1 => HeadingType::H1,
            2 => HeadingType::H2,
            3 => HeadingType::H3,
            4 => HeadingType::H4,
            5 => HeadingType::H5,
            6 => HeadingType::H6,
            _ => {panic!()},
        },
        content: data,
    }
}

fn read_content() {
    let file = File::open("/home/bourbon/dev/Inkspace/static/test.md")
        .expect("Failed to read Markdown file");

    let reader = BufReader::new(file);
    for line in reader.lines(){
        println!("{:?}", line);
    }
}

pub fn lex() {
    read_content();
}
