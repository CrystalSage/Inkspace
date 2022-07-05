use core::fmt;
use std::{fs, vec};

// Types of HTML tags we can have. 
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

struct PostDetails {
    date_published: String,
    reading_time_minutes: String,
    tags: Vec<String>,
}

struct Post {
    title: String,
    description: String,
    url: String,
    details: PostDetails,
}

// TODO: Somehow make this tidy.
fn generate_list(list_type: List, list_elements: Vec<String>) -> Vec<String> {
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

fn generate_div(classes: String) -> String {
    format!("<div class=\"{classes}\">")
}


// CALLME
pub fn call_generator() {
    construct_blog();
}


// ============================================================================
// # BLOG BUILDING FUNCTIONS BELOW
// ============================================================================

// Parent function
fn construct_blog() {
    let post: Post = Post { 
        title: "The curse of strong typing".into(),
        description: "Vg unccrarq jura V yrnfg rkcrpgrq vg.".into(),
        url: "/post1".into(),
        details: PostDetails { 
            date_published: "Jun 01, 2022".into(),
            reading_time_minutes: "69".into(),
            tags: vec!["rust".into(), "async".into()],
        },
    };

    construct_post(post);
}

fn fetch_posts() {
    todo!();
}

// List all the posts 
fn construct_post_container(posts: Vec<Post>) {
    let container_title = generate_heading(Heading::H1, "Recent articles".to_string());

    for post in posts {
        construct_post(post);
    }
}

// Generate a post
fn construct_post(post: Post) {
    let post_title: String = generate_heading(Heading::H1, post.title);
    let post_description: String = generate_paragraph(post.description);
    let post_link: String = generate_anchor(post.url, "Read more".to_string());
    let post_details = generate_list(List::Unordered, vec![post.details.date_published, post.details.reading_time_minutes + " minutes long", post.details.tags.join(" ")]);
}

// TODO: Add indents in future.
// TODO: Find a better way to do this.
pub fn generate_boilerplate() {
    let mut boilerplate: Vec<String> = Vec::new();
    boilerplate.push("<!DOCTYPE html>".to_string());
    boilerplate.push("<html>".to_string());
    boilerplate.push("<body>".to_string());
    boilerplate.push("</body>".to_string());
    boilerplate.push("</html>".to_string());
}

fn get_classnames() {
    let classes = fs::read_to_string("/home/bourbon/dev/Inkspace/static/classnames.json")
        .expect("Failed to read JSON file");

    let classes: serde_json::Value  = serde_json::from_str(&classes)
        .expect("Malformed JSON file");
}

#[cfg(test)]
mod tests {
    use crate::html_gen::generate_anchor;
    use super::{generate_heading, generate_paragraph, generate_list};
    use crate::fs;

    #[test]
    fn test_heading(){
        let heading: String = generate_heading( super::Heading::H1, 
                                        "This is a heading!".into());

        assert_eq!(heading, "<h1> This is a heading! </h1>".to_string());
    }

    #[test]
    fn test_anchor(){
        let anchor = generate_anchor( 
            "https://google.com".into(), 
            "Google".into());

        assert_eq!(anchor, "<a href='https://google.com'> Google </a>".to_string());
    }

    #[test]
    fn test_paragraph() {
        let paragraph = generate_paragraph(String::from("This is a paragraph!"));
        assert_eq!(paragraph, "<p> This is a paragraph! </p>");
    }

    #[test]
    fn test_list() {
        let list_elements: Vec<String> = vec!["Eggs".into(), "Bar".into() ];
        let list = generate_list(super::List::Unordered, list_elements);
        let expected: Vec<String> = vec![
            "<ul>".into(), 
            "<li> Eggs </li>".into(),
            "<li> Bar </li>".into(),
            "</ul>".into()
        ];

        assert_eq!(list, expected);
    }

    #[test]
    fn test_json() {
        let classes = fs::read_to_string("/home/bourbon/dev/Inkspace/static/classnames.json")
            .unwrap();
        let classes: serde_json::Value  = serde_json::from_str(&classes)
            .unwrap();

        assert_eq!(classes["Title"], "blog-heading");
    }
}
