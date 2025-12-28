use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Send > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < T > { self . raw . next_mut (& mut self . thread_local) . map (| entry | { * entry . present . get_mut () = false ; let cell = unsafe { & mut * entry . value . get () } ; let old_value = std :: mem :: replace (cell , MaybeUninit :: uninit ()) ; unsafe { old_value . assume_init () } }) } fn size_hint (& self) -> (usize , Option < usize >) { self . raw . size_hint_frozen (& self . thread_local) } }
}