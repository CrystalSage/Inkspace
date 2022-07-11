mod htmlGenerator {
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
}
