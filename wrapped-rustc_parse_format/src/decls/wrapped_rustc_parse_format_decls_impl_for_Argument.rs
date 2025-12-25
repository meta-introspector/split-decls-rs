use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'input> Argument<'input> {
    pub fn is_identifier(&self) -> bool {
        matches!(self.position, Position::ArgumentNamed(_))
            && self.format == FormatSpec::default()
    }
}
