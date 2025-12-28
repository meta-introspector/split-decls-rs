use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'tcx > Deref for BorrowckInferCtxt < 'tcx > { type Target = InferCtxt < 'tcx > ; fn deref (& self) -> & Self :: Target { & self . infcx } }
}