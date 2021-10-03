use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub output: String,
    pub passes: usize,
    pub threads: Option<usize>,
}
