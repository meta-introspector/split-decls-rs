use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type OptionArcdynSyntaxExtensionTraitDRT < DRT > = Option < Arc < dyn SyntaxExtensionTrait < DRT > > > ;
}