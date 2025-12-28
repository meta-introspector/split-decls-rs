use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
unsafe fn spawn_job < F > (func : F , registry : & Arc < Registry >) -> JobRef where F : FnOnce () + Send + 'static , { registry . increment_terminate_count () ; HeapJob :: new (Tlv :: null () , { let registry = Arc :: clone (registry) ; move | _ | { registry . catch_unwind (func) ; registry . terminate () ; } }) . into_static_job_ref () }
}