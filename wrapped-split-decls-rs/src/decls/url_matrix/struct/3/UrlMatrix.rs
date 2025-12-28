use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " URL Matrix - Collection of RDF URL blobs as a compressible matrix"] # [derive (Debug , Clone)] pub struct UrlMatrix { # [doc = " Matrix of URL blobs (rows = states, cols = features)"] pub matrix : Vec < Vec < f64 > > , # [doc = " URL blob sources"] pub urls : Vec < String > , # [doc = " Feature names (macro names, types, etc.)"] pub features : Vec < String > , # [doc = " Eigenform compression"] pub eigenform : Option < EigenForm > , }