use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Either a stack array with `length <= N` or a heap array"] # [doc = " whose pointer and capacity are stored here."] # [doc = ""] # [doc = " We store a `NonNull<T>` instead of a `*mut T`, so that"] # [doc = " niche-optimization can be performed and the type is covariant"] # [doc = " with respect to `T`."] # [repr (C)] pub union RawSmallVec < T , const N : usize > { inline : ManuallyDrop < MaybeUninit < [T ; N] > > , heap : (NonNull < T > , usize) , }