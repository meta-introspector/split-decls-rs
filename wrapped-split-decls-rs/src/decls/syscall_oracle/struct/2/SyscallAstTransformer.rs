use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct SyscallAstTransformer { interceptor : SyscallInterceptor , transformations : HashMap < String , usize > , }
}