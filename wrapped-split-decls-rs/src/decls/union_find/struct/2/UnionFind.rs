use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Simple implementation of a union-find data structure, i.e. a disjoint-set"] # [doc = " forest."] # [derive (Debug)] pub struct UnionFind < Key : Idx > { table : IndexVec < Key , UnionFindEntry < Key > > , }