use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Build the mapping of field names to fields. This allows attributes to peek values from"] # [doc = " other fields."] pub (super) fn build_field_mapping (variant : & VariantInfo < '_ >) -> HashMap < String , TokenStream > { let mut fields_map = FieldMap :: new () ; for binding in variant . bindings () { if let Some (ident) = & binding . ast () . ident { fields_map . insert (ident . to_string () , quote ! { # binding }) ; } } fields_map }