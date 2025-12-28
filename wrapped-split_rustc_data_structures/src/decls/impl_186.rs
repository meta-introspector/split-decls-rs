macro_rules! deps {
    () => {
        Annotation!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        # [doc = " The empty annotation, which does nothing."] impl Annotation for () { fn merge_reached (self , _other : Self) -> Self { () } fn merge_scc (self , _other : Self) -> Self { () } }
    };
}

impl_186!();