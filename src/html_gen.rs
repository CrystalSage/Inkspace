enum Heading{
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

fn generate_heading(heading: Heading, text_content: String) -> String{
    match heading {
        Heading::H1 => {
            let heading = "<h1>".to_owned() + &text_content + "</h1>";
            heading.to_string()
        }
        Heading::H2 => {
            let heading = "<h2>".to_owned() + &text_content + "</h2>";
            heading.to_string()
        }
        Heading::H3 => {
            let heading = "<h3>".to_owned() + &text_content + "</h3>";
            heading.to_string()
        }
        Heading::H4 => {
            let heading = "<h4>".to_owned() + &text_content + "</h4>";
            heading.to_string()
        }
        Heading::H5 => {
            let heading = "<h5>".to_owned() + &text_content + "</h5>";
            heading.to_string()
        }
        Heading::H6 => {
            let heading = "<h6>".to_owned() + &text_content + "</h6>";
            heading.to_string()
        }
    }
}

pub fn call_generator() -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let data: String = generate_heading(Heading::H1, "Hello Bourbon!".to_string());

    content.push(data);

    return content;
}
