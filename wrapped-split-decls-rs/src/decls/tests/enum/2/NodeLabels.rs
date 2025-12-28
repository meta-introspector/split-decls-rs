use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum NodeLabels < L > { AllNodesLabelled (Vec < L >) , UnlabelledNodes (usize) , SomeNodesLabelled (Vec < Option < L > >) , }
}