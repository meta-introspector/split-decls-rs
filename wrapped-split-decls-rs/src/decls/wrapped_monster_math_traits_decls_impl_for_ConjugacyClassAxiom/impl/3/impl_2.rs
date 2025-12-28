use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl ConjugacyClassAxiom for DummyConjugacyClassAxiom { fn get_canonical_class_count (& self) -> u32 { 194 } fn is_canonical_transformation_type (& self , transformation_type : & str) -> bool { transformation_type . starts_with ("canonical_") } fn validate_transformation_history (& self , _declaration : & Declaration , transformation_type : & str ,) -> bool { self . is_canonical_transformation_type (transformation_type) } }