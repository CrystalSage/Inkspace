enum Heading{
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

fn generate_paragraph(text_content: String) -> String {
    format!("<p> {text_content} </p>")
}

fn generate_anchor(href: String, text_content: String) -> String {
    format!("<a href='{href}'> {text_content} </a>")
}

fn generate_heading(heading: Heading, text_content: String) -> String{
    match heading {
        Heading::H1 => {
            format!("<h1> {} </h1>", text_content)
        }
        Heading::H2 => {
            format!("<h2> {} </h2>", text_content)
        }
        Heading::H3 => {
            format!("<h3> {} </h3>", text_content)
        }
        Heading::H4 => {
            format!("<h4> {} </h4>", text_content)
        }
        Heading::H5 => {
            format!("<h5> {} </h5>", text_content)
        }
        Heading::H6 => {
            format!("<h6> {} </h6>", text_content)
        }
    }
}

pub fn call_generator() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let heading: String = generate_heading(Heading::H1, "Hello Bourbon!".to_string());
    let paragraph: String = generate_paragraph("This is a paragraph!".to_string());
    let anchor: String = generate_anchor(
        "https://google.com".to_string(), 
        "Google".to_string());

    content.push(heading);
    content.push(paragraph);
    content.push(anchor);
    dbg!(&content);

    return content;
}
