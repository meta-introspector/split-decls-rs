use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > Deref for Layout < 'a > { type Target = & 'a LayoutData < FieldIdx , VariantIdx > ; fn deref (& self) -> & & 'a LayoutData < FieldIdx , VariantIdx > { & self . 0 . 0 } }