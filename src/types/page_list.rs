use super::page::Page;

pub struct PageList {
    total_count: u64,
    pages: Vec<Page>,
}