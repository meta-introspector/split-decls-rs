use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl HasFieldMap for DiagnosticDeriveVariantBuilder { fn get_field_binding (& self , field : & String) -> Option < & TokenStream > { self . field_map . get (field) } }