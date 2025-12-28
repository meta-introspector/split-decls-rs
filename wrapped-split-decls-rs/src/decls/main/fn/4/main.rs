use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () { let code = r#"fn example() {
    std::fs::write("test.txt", "data").unwrap();
    std::fs::read_to_string("test.txt").unwrap();
}"# ; let annotated = code . replace ("std::fs::write" , & format ! ("{}\n    std::fs::write" , syscallclippy ! ("write"))) . replace ("std::fs::read" , & format ! ("{}\n    std::fs::read" , syscallclippy ! ("read"))) ; println ! ("Original code:\n{}\n" , code) ; println ! ("Annotated code:\n{}" , annotated) ; fs :: write ("annotated_code.rs" , & annotated) . unwrap () ; println ! ("\n✅ 8-level recursive syscall annotations added!") ; println ! ("✅ File written: annotated_code.rs") ; }
}