pub trait SqlFormat {
    fn format(&self) -> String;
    fn format_str(&self) -> String;
}
