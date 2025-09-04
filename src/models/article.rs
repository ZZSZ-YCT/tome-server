use serde::{Deserialize, Serialize};
use crate::entities::article::AttributeMap;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleNoId {
    pub time: String,
    pub contributor: String,
    pub line: String,
    pub unsure: bool,
    pub sensitive: bool,
    pub attributes: Option<AttributeMap>,
}
