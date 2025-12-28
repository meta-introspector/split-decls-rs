macro_rules! deps {
    () => {
        TrackedStruct!();
        AllowedPersistOptions!();
        AllowedOptions!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl AllowedOptions for TrackedStruct { const RETURNS : bool = false ; const SPECIFY : bool = false ; const NO_EQ : bool = false ; const DEBUG : bool = true ; const NO_LIFETIME : bool = false ; const NON_UPDATE_RETURN_TYPE : bool = false ; const SINGLETON : bool = true ; const DATA : bool = true ; const DB : bool = false ; const CYCLE_FN : bool = false ; const CYCLE_INITIAL : bool = false ; const CYCLE_RESULT : bool = false ; const LRU : bool = false ; const CONSTRUCTOR_NAME : bool = true ; const ID : bool = false ; const REVISIONS : bool = false ; const HEAP_SIZE : bool = true ; const SELF_TY : bool = false ; const PERSIST : AllowedPersistOptions = AllowedPersistOptions :: AllowedValue ; }
    };
}

impl_97!();