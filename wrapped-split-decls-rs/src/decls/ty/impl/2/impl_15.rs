use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , Ty > AsRef < LayoutData < FieldIdx , VariantIdx > > for TyAndLayout < 'a , Ty > { fn as_ref (& self) -> & LayoutData < FieldIdx , VariantIdx > { & * self . layout . 0 . 0 } }
}