use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < S , Ctx > SrcToken < RawConverter < '_ , Ctx > , S > for usize { fn kind (& self , ctx : & RawConverter < '_ , Ctx >) -> SyntaxKind { ctx . lexed . kind (* self) } fn to_char (& self , ctx : & RawConverter < '_ , Ctx >) -> Option < char > { ctx . lexed . text (* self) . chars () . next () } fn to_text (& self , ctx : & RawConverter < '_ , Ctx >) -> SmolStr { ctx . lexed . text (* self) . into () } }
}