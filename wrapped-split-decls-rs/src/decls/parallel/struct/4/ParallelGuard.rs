use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A guard used to hold panics that occur during a parallel section to later by unwound."] # [doc = " This is used for the parallel compiler to prevent fatal errors from non-deterministically"] # [doc = " hiding errors by ensuring that everything in the section has completed executing before"] # [doc = " continuing with unwinding. It's also used for the non-parallel code to ensure error message"] # [doc = " output match the parallel compiler for testing purposes."] pub struct ParallelGuard { panic : Mutex < Option < IntoDynSyncSend < Box < dyn Any + Send + 'static > > > > , }
}