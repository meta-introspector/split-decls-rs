use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Sequence { # [doc = " Create a new empty [`Sequence`]"] pub fn new () -> Self { Self :: default () } # [doc = " Not for public consumption, but it must be public so the generated code"] # [doc = " can call it."] # [doc (hidden)] pub fn next_handle (& mut self) -> SeqHandle { let handle = SeqHandle { inner : self . inner . clone () , seq : self . next_seq , } ; self . next_seq += 1 ; handle } }
}