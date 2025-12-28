use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'parent , 'a > HasFieldMap for SubdiagnosticDeriveVariantBuilder < 'parent , 'a > { fn get_field_binding (& self , field : & String) -> Option < & TokenStream > { self . fields . get (field) } }