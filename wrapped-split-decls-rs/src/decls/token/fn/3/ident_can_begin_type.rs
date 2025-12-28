use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn ident_can_begin_type (name : Symbol , span : Span , is_raw : IdentIsRaw) -> bool { let ident_token = Token :: new (Ident (name , is_raw) , span) ; ! ident_token . is_reserved_ident () || ident_token . is_path_segment_keyword () || [kw :: Underscore , kw :: For , kw :: Impl , kw :: Fn , kw :: Unsafe , kw :: Extern , kw :: Typeof , kw :: Dyn] . contains (& name) }