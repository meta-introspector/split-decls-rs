use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Debug , PartialEq , Eq)] pub struct ClosureCapture < 'db > { owner : DefWithBodyId , closure : InternedClosureId , capture : hir_ty :: CapturedItem < 'db > , }