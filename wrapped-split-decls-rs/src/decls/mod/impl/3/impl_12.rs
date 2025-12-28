use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < G > Iterator for DepthFirstSearch < G > where G : DirectedGraph + Successors , { type Item = G :: Node ; fn next (& mut self) -> Option < G :: Node > { let DepthFirstSearch { stack , visited , graph } = self ; let n = stack . pop () ? ; stack . extend (graph . successors (n) . filter (| & m | visited . insert (m))) ; Some (n) } }
}