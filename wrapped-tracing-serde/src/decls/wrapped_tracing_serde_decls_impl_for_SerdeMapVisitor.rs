use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S> SerdeMapVisitor<S>
where
    S: SerializeMap,
{
    /// Create a new map visitor.
    pub fn new(serializer: S) -> Self {
        Self {
            serializer,
            state: Ok(()),
        }
    }
    /// Completes serializing the visited object, returning `Ok(())` if all
    /// fields were serialized correctly, or `Error(S::Error)` if a field could
    /// not be serialized.
    pub fn finish(self) -> Result<S::Ok, S::Error> {
        self.state?;
        self.serializer.end()
    }
    /// Completes serializing the visited object, returning ownership of the underlying serializer
    /// if all fields were serialized correctly, or `Err(S::Error)` if a field could not be
    /// serialized.
    pub fn take_serializer(self) -> Result<S, S::Error> {
        self.state?;
        Ok(self.serializer)
    }
}
