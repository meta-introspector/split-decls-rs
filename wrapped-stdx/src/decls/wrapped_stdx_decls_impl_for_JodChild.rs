use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl JodChild {
    pub fn spawn(mut command: Command) -> sio::Result<Self> {
        command.spawn().map(Self)
    }
    #[must_use]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn into_inner(self) -> std::process::Child {
        unsafe { std::mem::transmute::<Self, std::process::Child>(self) }
    }
}
