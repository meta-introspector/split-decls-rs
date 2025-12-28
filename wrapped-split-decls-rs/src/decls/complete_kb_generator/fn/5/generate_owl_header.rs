use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn generate_owl_header () -> String { format ! (r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix lmdfb: <http://split-decls-rs.org/lmdfb#> .
@prefix rustc: <http://rust-lang.org/rustc#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

<http://split-decls-rs.org/lmdfb> rdf:type owl:Ontology ;
    rdfs:label "Rustc LMDFB Knowledge Base" ;
    rdfs:comment "Complete semantic model of Rust compiler ecosystem" ;
    owl:versionInfo "1.0" .

# Core Classes
lmdfb:RustcComponent rdf:type owl:Class .
lmdfb:CompilerPhase rdf:type owl:Class .
lmdfb:SemanticPattern rdf:type owl:Class .
lmdfb:MathematicalStructure rdf:type owl:Class .
lmdfb:EcosystemRelation rdf:type owl:Class .

"#) }
}