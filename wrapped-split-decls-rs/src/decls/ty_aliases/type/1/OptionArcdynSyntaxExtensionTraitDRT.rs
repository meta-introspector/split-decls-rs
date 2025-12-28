use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type OptionArcdynSyntaxExtensionTraitDRT < DRT > = Option < Arc < dyn SyntaxExtensionTrait < DRT > > > ;