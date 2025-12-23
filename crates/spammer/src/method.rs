pub enum Method {
    Raw,
    Bundle,
    Private,
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::Raw => "raw",
            Method::Bundle => "bundle",
            Method::Private => "private",
        }
    }
}
