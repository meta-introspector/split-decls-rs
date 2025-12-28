macro_rules! deps {
    () => {
        Ty!();
        Interner!();
        Component!();
        OutlivesCollector!();
    };
}

macro_rules! push_outlives_components {
    () => {
        deps!();
        # [doc = " Push onto `out` all the things that must outlive `'a` for the condition"] # [doc = " `ty0: 'a` to hold. Note that `ty0` must be a **fully resolved type**."] pub fn push_outlives_components < I : Interner > (cx : I , ty : I :: Ty , out : & mut SmallVec < [Component < I > ; 4] > ,) { ty . visit_with (& mut OutlivesCollector { cx , out , visited : Default :: default () }) ; }
    };
}

push_outlives_components!()