use super::types::node::Node;

pub struct CreatePage {
    access_token: String,
    // 1 - 256 characters
    title: String,
    // 0 - 128 characters
    author_name: String,
    // 0 - 512 characters
    author_url: String,
    content: Vec<Node>
}

// returns Page object