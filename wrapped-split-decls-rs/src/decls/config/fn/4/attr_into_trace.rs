use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: attr_into_trace");
pub fn attr_into_trace (mut attr : ast :: Attribute , trace_name : Symbol) -> ast :: Attribute { match & mut attr . kind { ast :: AttrKind :: Normal (normal) => { let ast :: NormalAttr { item , tokens } = & mut * * normal ; item . path . segments [0] . ident . name = trace_name ; * tokens = Some (ast :: tokenstream :: LazyAttrTokenStream :: new_direct (ast :: tokenstream :: AttrTokenStream :: default ())) ; } ast :: AttrKind :: DocComment (..) => unreachable ! () , } attr }
}