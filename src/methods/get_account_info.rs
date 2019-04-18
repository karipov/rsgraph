

pub struct GetAccountInfo {
    access_token: String,
    // default: [“short_name”,“author_name”,“author_url”]
    // also available:  auth_url, page_count
    fields: Vec<String>,
}

// must return Account object