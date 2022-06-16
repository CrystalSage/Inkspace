// Markdown parser. Because why not?
//
// Roadmap:
//   - Headings
//   - Format specifiers
//
// Architecture:
//   - Parse token by token
//

struct Token<T> {
    span: [u32;2],
    contents: <T>,
}

pub fn parse_heading() {
}

fn main() {

}
