use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: execs");
# [doc = " Run and verify a process, see [`Execs`]"] pub fn execs () -> Execs { Execs { ran : false , process_builder : None , expect_stdin : None , expect_exit_code : Some (0) , expect_stdout_data : None , expect_stderr_data : None , expect_stdout_contains : Vec :: new () , expect_stderr_contains : Vec :: new () , expect_stdout_not_contains : Vec :: new () , expect_stderr_not_contains : Vec :: new () , expect_stderr_with_without : Vec :: new () , stream_output : false , assert : compare :: assert_e2e () , } }
}