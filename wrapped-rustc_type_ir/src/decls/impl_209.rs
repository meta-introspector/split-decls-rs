macro_rules! deps {
    () => {
        Interner!();
        TypeWalker!();
        GenericArg!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < I : Interner > Iterator for TypeWalker < I > { type Item = I :: GenericArg ; fn next (& mut self) -> Option < I :: GenericArg > { debug ! ("next(): stack={:?}" , self . stack) ; loop { let next = self . stack . pop () ? ; self . last_subtree = self . stack . len () ; if self . visited . insert (next) { push_inner :: < I > (& mut self . stack , next) ; debug ! ("next: stack={:?}" , self . stack) ; return Some (next) ; } } } }
    };
}

impl_209!();