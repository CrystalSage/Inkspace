enum Tags {
    Anchor,
    Heading,
    OrdList,
    Paragraph,
    UnOrdList,
}

enum Heading{
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

enum List {
    Ordered,
    Unordered,
}

// TODO: Somehow optimize this
fn generate_unordered_list(list_type: List, list_elements: Vec<String>) -> Vec<String> {
    let mut list_content: Vec<String> = vec!["".to_string(); 2 + list_elements.len()];

    match list_type {
        List::Ordered => { 
            list_content[0] = format!("<ol>");
            list_content[2 + list_elements.len() - 1] = format!("</ol>");

        },
        List::Unordered => { 
            list_content[0] = format!("<ul>");
            list_content[2 + list_elements.len() - 1] = format!("</ul>");
        },
    }

    for (idx, element) in list_elements.iter().enumerate() {
        list_content[1 + idx] = format!("<li> {} </li>", element); 
    }


    return list_content;
}

fn generate_paragraph(text_content: String) -> String {
    format!("<p> {text_content} </p>")
}

fn generate_anchor(href: String, text_content: String) -> String {
    format!("<a href='{href}'> {text_content} </a>")
}

fn generate_heading(heading: Heading, text_content: String) -> String{
    match heading {
        Heading::H1 => { format!("<h1> {} </h1>", text_content) }
        Heading::H2 => { format!("<h2> {} </h2>", text_content) }
        Heading::H3 => { format!("<h3> {} </h3>", text_content) }
        Heading::H4 => { format!("<h4> {} </h4>", text_content) }
        Heading::H5 => { format!("<h5> {} </h5>", text_content) }
        Heading::H6 => { format!("<h6> {} </h6>", text_content) }
    }
}

pub fn call_generator() -> Vec<String> {
    //let content: Vec<String> = Vec::new();
    let list = generate_unordered_list(List::Ordered, vec!["eggs".to_string(),"bar".to_string()]);

    return list;
}
