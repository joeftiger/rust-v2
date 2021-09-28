use crate::scene::{Emitter, Receiver};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SceneObject {
    Emitter(Emitter),
    Receiver(Receiver),
}
