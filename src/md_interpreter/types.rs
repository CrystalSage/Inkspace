// A token that is produced after lexing.
pub struct Token {
    // Data of the token
    content: String,
    // Type of Markdown element
    token_type: MDType, 
    // Span of the content: [from, to]
    span: [u8; 2],
}

pub enum MDType {
    Heading,
    Link,
    Paragraph,
    Break,
}


#[derive(Debug)]
pub enum HeadingType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug)]
pub struct Heading {
    level: HeadingType,
    content: String,
}
