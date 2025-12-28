use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn count_type_complexity (ty : & syn :: Type) -> u8 { match ty { syn :: Type :: Path (path) => { let segments = & path . path . segments ; let base_complexity = if segments . len () > 2 { 2 } else { 1 } ; let generic_complexity = segments . iter () . map (| seg | match & seg . arguments { syn :: PathArguments :: AngleBracketed (args) => args . args . len () as u8 , _ => 0 , }) . sum :: < u8 > () ; base_complexity + generic_complexity } , syn :: Type :: Reference (_) => 1 , syn :: Type :: Tuple (tuple) => tuple . elems . len () as u8 , _ => 1 , } }
}