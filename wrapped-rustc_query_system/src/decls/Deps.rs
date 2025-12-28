macro_rules! deps {
    () => {
        TaskDepsRef!();
        DepKind!();
    };
}

macro_rules! Deps {
    () => {
        deps!();
        pub trait Deps : DynSync { # [doc = " Execute the operation with provided dependencies."] fn with_deps < OP , R > (deps : TaskDepsRef < '_ > , op : OP) -> R where OP : FnOnce () -> R ; # [doc = " Access dependencies from current implicit context."] fn read_deps < OP > (op : OP) where OP : for < 'a > FnOnce (TaskDepsRef < 'a >) ; fn name (& self , dep_kind : DepKind) -> & 'static str ; # [doc = " We use this for most things when incr. comp. is turned off."] const DEP_KIND_NULL : DepKind ; # [doc = " We use this to create a forever-red node."] const DEP_KIND_RED : DepKind ; # [doc = " We use this to create a side effect node."] const DEP_KIND_SIDE_EFFECT : DepKind ; # [doc = " We use this to create the anon node with zero dependencies."] const DEP_KIND_ANON_ZERO_DEPS : DepKind ; # [doc = " This is the highest value a `DepKind` can have. It's used during encoding to"] # [doc = " pack information into the unused bits."] const DEP_KIND_MAX : u16 ; }
    };
}

Deps!();