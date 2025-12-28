use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < C > CompilerHost < C > for MockCompilerHost { fn run_compiler_callbacks (& self , _args : Vec < String > , _callbacks : & mut C) { println ! ("MockCompilerHost::run_compiler_callbacks called") ; } }