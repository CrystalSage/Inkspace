// A token that is produced after lexing.
struct Token {
    // Data of the content
    content: String,
    // Span of the content [from, to]
    span: [u8; 2],
}

enum HeadingType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

struct Heading {
    level: HeadingType,
    content: String,
}

fn analyze_heading(line: String) -> Heading {
    let mut depth: usize = 0;
    let content_chars: Vec<char> = line.chars().collect();

    // TODO: Maybe use a `Peekable` iterator if we encounter any performance
    // issues.
    for marker in &content_chars {
        match marker {
            '#' => { depth = depth + 1; }
            _ => {}
        }
    }

    let data: String = content_chars[depth..].into_iter().collect();
    let data: String = data.trim_start().to_string();

    Heading {
        level: HeadingType::H1,
        content: data,
    }
}

pub fn lex() {
    dbg!(analyze_heading("## Moja Global".to_string()));
}
