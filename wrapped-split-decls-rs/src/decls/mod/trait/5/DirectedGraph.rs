use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait DirectedGraph { type Node : Idx ; # [doc = " Returns the total number of nodes in this graph."] # [doc = ""] # [doc = " Several graph algorithm implementations assume that every node ID is"] # [doc = " strictly less than the number of nodes, i.e. nodes are densely numbered."] # [doc = " That assumption allows them to use `num_nodes` to allocate per-node"] # [doc = " data structures, indexed by node."] fn num_nodes (& self) -> usize ; # [doc = " Iterates over all nodes of a graph in ascending numeric order."] # [doc = ""] # [doc = " Assumes that nodes are densely numbered, i.e. every index in"] # [doc = " `0..num_nodes` is a valid node."] fn iter_nodes (& self ,) -> impl Iterator < Item = Self :: Node > + DoubleEndedIterator + ExactSizeIterator { (0 .. self . num_nodes ()) . map (< Self :: Node as Idx > :: new) } }
}