use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn generate_rdf_ttl (rdf_state : & RdfStateMachine) -> String { let mut ttl = String :: from ("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n") ; ttl . push_str ("@prefix ast: <http://split-decls.rs/ast#> .\n") ; ttl . push_str ("@prefix exec: <http://split-decls.rs/execution#> .\n\n") ; for triple in & rdf_state . triples { ttl . push_str (& format ! ("ast:{} ast:{} \"{}\" .\n" , triple . subject . replace ("::" , "_") , triple . predicate . replace (":" , "_") , triple . object)) ; } ttl }