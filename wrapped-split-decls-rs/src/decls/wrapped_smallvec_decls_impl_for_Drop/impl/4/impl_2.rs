use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T , const N : usize > Drop for IntoIter < T , N > { fn drop (& mut self) { unsafe { let is_zst = size_of :: < T > () == 0 ; let on_heap = self . end . on_heap (is_zst) ; let begin = self . begin ; let end = self . end . value (is_zst) ; let ptr = self . as_mut_ptr () ; let _drop_dealloc = if on_heap { let capacity = self . raw . heap . 1 ; Some (DropDealloc { ptr : NonNull :: new_unchecked (ptr as * mut u8) , size_bytes : capacity * size_of :: < T > () , align : align_of :: < T > () , }) } else { None } ; core :: ptr :: slice_from_raw_parts_mut (ptr . add (begin) , end - begin) . drop_in_place () ; } } }
}