use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn allocate_bucket < T > (size : usize) -> * mut Entry < T > { Box :: into_raw ((0 .. size) . map (| _ | Entry :: < T > { present : AtomicBool :: new (false) , value : UnsafeCell :: new (MaybeUninit :: uninit ()) , }) . collect () ,) as * mut _ }