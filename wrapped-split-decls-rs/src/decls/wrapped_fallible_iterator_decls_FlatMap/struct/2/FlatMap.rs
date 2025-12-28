use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An iterator which maps each element to another iterator, yielding those iterator's elements."] # [derive (Clone , Debug)] pub struct FlatMap < I , U , F > where U : IntoFallibleIterator , { it : Map < I , F > , cur : Option < U :: IntoFallibleIter > , }