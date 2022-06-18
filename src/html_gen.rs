enum Heading{
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

fn generate_heading(heading: Heading){
    match heading {
        Heading::H1 => "<h1>",
        Heading::H2 => "<h2>",
        Heading::H3 => "<h3>",
        Heading::H4 => "<h4>",
        Heading::H5 => "<h5>",
        Heading::H6 => "<h6>",
    }
}
