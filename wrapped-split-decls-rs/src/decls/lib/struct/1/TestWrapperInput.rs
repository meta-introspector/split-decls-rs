use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct TestWrapperInput { wrapper_path : syn :: Path , test_name : Ident , }
}