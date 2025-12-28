use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: C");
# [allow (non_snake_case)] fn C < OF , BF , O > (of : OF , bf : BF) -> ClosureObligationProcessor < OF , BF , O , & 'static str > where OF : FnMut (& mut O) -> ProcessResult < O , & 'static str > , BF : FnMut (& [O]) , { ClosureObligationProcessor { process_obligation : of , _process_backedge : bf , marker : PhantomData , } }
}