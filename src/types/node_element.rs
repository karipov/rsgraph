use std::collections::HashMap;
use super::node::Node;

pub struct NodeElement {
    // available tags: a, aside, b, blockquote, br, code, em,
    tag: String,
    // available attrs: href, src
    attrs: Option<HashMap<String, i32>>,
    children: Option<Vec<Node>>

}