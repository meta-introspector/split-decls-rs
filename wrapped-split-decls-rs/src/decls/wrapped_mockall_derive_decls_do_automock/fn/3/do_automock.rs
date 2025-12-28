use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn do_automock (attrs : TokenStream , input : TokenStream) -> TokenStream { cfg_if ! { if # [cfg (reprocheck)] { let ts_a = do_automock_once (attrs . clone () , input . clone ()) ; let ts_b = do_automock_once (attrs . clone () , input . clone ()) ; assert_eq ! (ts_a . to_string () , ts_b . to_string ()) ; } } do_automock_once (attrs , input) }
}