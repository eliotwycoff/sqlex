#[derive(Debug, Clone)]
pub struct Where {
    pub column: String,
    pub operator: String,
    pub value: String,
}
