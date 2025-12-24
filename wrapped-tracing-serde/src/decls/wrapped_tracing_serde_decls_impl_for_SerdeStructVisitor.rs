use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S: SerializeStruct> SerdeStructVisitor<S> {
    /// Completes serializing the visited object, returning `Ok(())` if all
    /// fields were serialized correctly, or `Error(S::Error)` if a field could
    /// not be serialized.
    pub fn finish(self) -> Result<S::Ok, S::Error> {
        self.state?;
        self.serializer.end()
    }
}
