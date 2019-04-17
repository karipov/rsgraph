use super::node::Node;

pub struct Page {
    path: String,
    url: String,
    title: String,
    description: String,
    author_name: Option<String>,
    author_url: Option<String>,
    image_url: Option<String>,
    content: Vec<Node>,

}