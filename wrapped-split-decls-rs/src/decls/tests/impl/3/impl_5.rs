use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > TypedArena < T > { # [doc = " Clears the arena. Deallocates all but the longest chunk which may be reused."] fn clear (& mut self) { unsafe { let mut chunks_borrow = self . chunks . borrow_mut () ; if let Some (mut last_chunk) = chunks_borrow . last_mut () { self . clear_last_chunk (& mut last_chunk) ; let len = chunks_borrow . len () ; for mut chunk in chunks_borrow . drain (.. len - 1) { chunk . destroy (chunk . entries) ; } } } } }