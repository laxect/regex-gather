pub struct Predicate {
    pub pattern: String,
    pub preview_left: String,
    pub preview_right: String,
    pub highlight: String,
}

impl Predicate {
    pub fn new(pattern: String, left: String, right: String, highlight: String) -> Self {
        Self {
            pattern,
            preview_left: left,
            preview_right: right,
            highlight,
        }
    }
}

impl AsRef<String> for Predicate {
    fn as_ref(&self) -> &String {
        &self.pattern
    }
}
