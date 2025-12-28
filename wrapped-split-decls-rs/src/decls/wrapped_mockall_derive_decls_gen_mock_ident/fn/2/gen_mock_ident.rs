use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Generate a mock identifier from the regular one: eg \"Foo\" => \"MockFoo\""] fn gen_mock_ident (ident : & Ident) -> Ident { format_ident ! ("Mock{}" , ident) }
}