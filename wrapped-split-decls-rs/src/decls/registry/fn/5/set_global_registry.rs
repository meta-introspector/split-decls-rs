use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: set_global_registry");
# [doc = " Starts the worker threads (if that has not already happened)"] # [doc = " by creating a registry with the given callback."] fn set_global_registry < F > (registry : F) -> Result < & 'static Arc < Registry > , ThreadPoolBuildError > where F : FnOnce () -> Result < Arc < Registry > , ThreadPoolBuildError > , { let mut result = Err (ThreadPoolBuildError :: new (ErrorKind :: GlobalPoolAlreadyInitialized)) ; THE_REGISTRY_SET . call_once (| | { result = registry () . map (| registry : Arc < Registry > | { unsafe { ptr :: addr_of_mut ! (THE_REGISTRY) . write (Some (registry)) ; (* ptr :: addr_of ! (THE_REGISTRY)) . as_ref () . unwrap_unchecked () } }) }) ; result }
}