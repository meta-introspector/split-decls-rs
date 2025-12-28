macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! BreakableTarget {
    () => {
        deps!();
        # [doc = " The target of an expression that breaks out of a scope"] # [derive (Clone , Copy , Debug)] pub (crate) enum BreakableTarget { Continue (region :: Scope) , Break (region :: Scope) , Return , }
    };
}

BreakableTarget!()