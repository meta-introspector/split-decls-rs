macro_rules! deps {
    () => {
        TrackedFn!();
        AllowedOptions!();
        AllowedPersistOptions!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl AllowedOptions for TrackedFn { const RETURNS : bool = true ; const SPECIFY : bool = true ; const NO_EQ : bool = true ; const DEBUG : bool = false ; const NO_LIFETIME : bool = false ; const NON_UPDATE_RETURN_TYPE : bool = true ; const SINGLETON : bool = false ; const DATA : bool = false ; const DB : bool = false ; const CYCLE_FN : bool = true ; const CYCLE_INITIAL : bool = true ; const CYCLE_RESULT : bool = true ; const LRU : bool = true ; const CONSTRUCTOR_NAME : bool = false ; const ID : bool = false ; const REVISIONS : bool = false ; const HEAP_SIZE : bool = true ; const SELF_TY : bool = true ; const PERSIST : AllowedPersistOptions = AllowedPersistOptions :: AllowedIdent ; }
    };
}

impl_78!()