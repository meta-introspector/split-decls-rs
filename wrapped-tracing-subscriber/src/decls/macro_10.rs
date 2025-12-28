macro_rules! deps {
    () => {
        Alt!();
    };
}

macro_rules! macro_10 {
    () => {
        deps!();
        feature ! { #! [feature = "std"] use super :: VisitWrite ; use std :: io ; impl < V > VisitWrite for Alt < V > where V : VisitWrite , { # [inline] fn writer (& mut self) -> & mut dyn io :: Write { self . 0 . writer () } } }
    };
}

macro_10!()