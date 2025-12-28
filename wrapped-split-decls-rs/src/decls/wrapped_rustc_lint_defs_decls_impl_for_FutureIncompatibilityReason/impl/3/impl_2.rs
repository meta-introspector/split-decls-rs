use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FutureIncompatibilityReason { pub fn edition (self) -> Option < Edition > { match self { Self :: EditionError (e) | Self :: EditionSemanticsChange (e) | Self :: EditionAndFutureReleaseError (e) | Self :: EditionAndFutureReleaseSemanticsChange (e) => Some (e) , FutureIncompatibilityReason :: FutureReleaseError | FutureIncompatibilityReason :: FutureReleaseSemanticsChange | FutureIncompatibilityReason :: Custom (_) => None , } } }