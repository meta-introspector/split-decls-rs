use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone)] pub struct DiagCtxtHandle < 'a > { dcx : & 'a DiagCtxt , # [doc = " Some contexts create `DiagCtxtHandle` with this field set, and thus all"] # [doc = " errors emitted with it will automatically taint when emitting errors."] tainted_with_errors : Option < & 'a Cell < Option < ErrorGuaranteed > > > , }