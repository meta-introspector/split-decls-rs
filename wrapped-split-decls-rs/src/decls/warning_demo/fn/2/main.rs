use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () { println ! ("🧪 Testing macro patch warnings...\n") ; let output = audit_execute ! (Command :: new ("echo") . arg ("Hello World") . output ()) . unwrap () ; println ! ("Result: {}" , String :: from_utf8_lossy (& output . stdout)) ; audit_execute ! (Command :: new ("date") . status ()) . unwrap () ; }