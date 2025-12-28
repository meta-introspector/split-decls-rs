use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T : Copy > AppendOnlyIndexVec < I , T > { pub fn new () -> Self { Self { vec : elsa :: sync :: LockFreeFrozenVec :: new () , _marker : PhantomData } } pub fn push (& self , val : T) -> I { let i = self . vec . push (val) ; I :: new (i) } pub fn get (& self , i : I) -> Option < T > { let i = i . index () ; self . vec . get (i) } }