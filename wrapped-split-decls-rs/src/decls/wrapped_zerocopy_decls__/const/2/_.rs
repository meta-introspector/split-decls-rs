use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
const _ : () = unsafe { unsafe_impl_known_layout ! (T : ? Sized + KnownLayout => # [repr (T :: MaybeUninit)] MaybeUninit < T >) } ;
}