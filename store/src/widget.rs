use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Widget {
    pub(crate) id: i32,
    pub(crate) name: &'static str
}
