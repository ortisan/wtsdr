#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url(String);

impl Url {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, &'static str> {
        let s = s.as_ref().trim();
        if !(s.starts_with("http://") || s.starts_with("https://")) {
            return Err("url must start with http:// or https://");
        }
        Ok(Url(s.to_owned()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
