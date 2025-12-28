use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct SyscallAstTransformer { interceptor : SyscallInterceptor , transformations : HashMap < String , usize > , }