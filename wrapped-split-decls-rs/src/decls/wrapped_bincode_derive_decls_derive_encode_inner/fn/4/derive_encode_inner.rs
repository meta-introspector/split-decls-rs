use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: derive_encode_inner");
fn derive_encode_inner (input : TokenStream) -> Result < TokenStream > { let parse = Parse :: new (input) ? ; let (mut generator , attributes , body) = parse . into_generator () ; let attributes = attributes . get_attribute :: < ContainerAttributes > () ? . unwrap_or_default () ; match body { Body :: Struct (body) => { derive_struct :: DeriveStruct { fields : body . fields , attributes , } . generate_encode (& mut generator) ? ; } Body :: Enum (body) => { derive_enum :: DeriveEnum { variants : body . variants , attributes , } . generate_encode (& mut generator) ? ; } } generator . export_to_file ("bincode" , "Encode") ; generator . finish () }
}