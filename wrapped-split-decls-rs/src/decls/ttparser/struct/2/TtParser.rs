use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Debug)] pub struct TtParser < 'a > { pub sess : & 'a ParseSess , pub token_tree : Cow < 'a , TokenStream > , pub idx : usize , pub dot2_is_not_dotdot : bool , pub macro_name : Ident , pub expansion_seqs : Vec < Box < [MatchedSeq] > > , }
}