use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , Ty > Deref for TyAndLayout < 'a , Ty > { type Target = & 'a LayoutData < FieldIdx , VariantIdx > ; fn deref (& self) -> & & 'a LayoutData < FieldIdx , VariantIdx > { & self . layout . 0 . 0 } }
}