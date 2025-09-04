use crate::entities::article::AttributeVec;

pub struct ArticleNoId {
    pub time: String,
    pub contributor: String,
    pub line: String,
    pub unsure: bool,
    pub sensitive: bool,
    pub attributes: Option<AttributeVec>,
}
