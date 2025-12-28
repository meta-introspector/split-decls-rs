use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn calculate_nested_depth (fields : & Fields) -> u8 { match fields { Fields :: Named (fields) => { fields . named . iter () . map (| f | count_type_complexity (& f . ty)) . max () . unwrap_or (0) } , Fields :: Unnamed (fields) => { fields . unnamed . iter () . map (| f | count_type_complexity (& f . ty)) . max () . unwrap_or (0) } , Fields :: Unit => 0 , } }
}