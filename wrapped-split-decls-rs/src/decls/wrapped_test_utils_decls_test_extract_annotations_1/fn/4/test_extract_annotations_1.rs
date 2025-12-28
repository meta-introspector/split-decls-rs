use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_extract_annotations_1");
# [test] fn test_extract_annotations_1 () { let text = stdx :: trim_indent (r#"
fn main() {
    let (x,     y) = (9, 2);
       //^ def  ^ def
    zoo + 1
} //^^^ type:
  //  | i32

// ^file
    "# ,) ; let res = extract_annotations (& text) . into_iter () . map (| (range , ann) | (& text [range] , ann)) . collect :: < Vec < _ > > () ; assert_eq ! (res [.. 3] , [("x" , "def" . into ()) , ("y" , "def" . into ()) , ("zoo" , "type:\ni32\n" . into ())]) ; assert_eq ! (res [3] . 0 . len () , 115) ; }
}