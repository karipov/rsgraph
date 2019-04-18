use super::node::Node;

pub struct Page {
    path: String,
    url: String,
    title: String,
    description: String,
    views: u64,
    author_name: Option<String>,
    author_url: Option<String>,
    image_url: Option<String>,
    content: Option<Vec<Node>>,
    can_edit: Option<bool>,
}