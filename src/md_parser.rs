enum MDType {
    Heading,
}

enum Heading {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

// TODO: Constrain this to `MDType` maybe
struct Token<T> {
    elem_type: T,
    content: String,
}

fn parse_heading() -> Token<Heading> {
    // Borken
    Token {
        elem_type: Heading::One,
        content: String::from("Hello"),
    }
}

pub fn call_parse_function() {
    parse_heading();
}
