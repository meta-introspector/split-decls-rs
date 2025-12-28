use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , PartialEq , Eq , Hash , Debug)] enum Callee < 'db > { Def (CallableDefId) , Closure (InternedClosureId , GenericArgs < 'db >) , CoroutineClosure (InternedCoroutineId , GenericArgs < 'db >) , FnPtr , FnImpl (FnTrait) , }