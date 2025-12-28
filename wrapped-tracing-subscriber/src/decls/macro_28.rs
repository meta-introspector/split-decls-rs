macro_rules! deps {
    () => {
        Messages!();
    };
}

macro_rules! macro_28 {
    () => {
        deps!();
        feature ! { #! [feature = "std"] use super :: VisitWrite ; use std :: io ; impl < V > VisitWrite for Messages < V > where V : VisitWrite , { # [inline] fn writer (& mut self) -> & mut dyn io :: Write { self . 0 . writer () } } }
    };
}

macro_28!();