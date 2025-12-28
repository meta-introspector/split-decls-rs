use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn run_with_stack (stack_size_in_mb : usize) { let pool = ThreadPoolBuilder :: new () . stack_size (stack_size_in_mb * 1024 * 1024) . build () . unwrap () ; pool . install (| | { # [cfg (unix)] disable_core () ; force_stack_overflow (32) ; }) ; }
}