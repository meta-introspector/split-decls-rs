use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Only keep `#[doc]` attrs."] fn scrub_attrs (attrs : & [Attribute]) -> Vec < Attribute > { attrs . into_iter () . cloned () . filter (| attr | { let ident = & attr . path () . segments [0] . ident ; ident == "doc" || ident == "must_use" }) . collect () }
}