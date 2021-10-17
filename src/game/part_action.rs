use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PartAction {
    Open,
    Close,
}
