use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Default)] pub struct AppendOnlyIndexVec < I : Idx , T : Copy > { vec : elsa :: sync :: LockFreeFrozenVec < T > , _marker : PhantomData < fn (& I) > , }