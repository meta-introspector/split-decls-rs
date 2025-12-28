use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ClosureObligationProcessor < OF , BF , O , E > { process_obligation : OF , _process_backedge : BF , marker : PhantomData < (O , E) > , }
}