macro_rules! deps {
    () => {
        JoinMap!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < K , V > JoinMap < K , V > { # [doc = " Creates a new empty `JoinMap`."] # [doc = ""] # [doc = " The `JoinMap` is initially created with a capacity of 0, so it will not"] # [doc = " allocate until a task is first spawned on it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tokio_util::task::JoinMap;"] # [doc = " let map: JoinMap<&str, i32> = JoinMap::new();"] # [doc = " ```"] # [inline] # [must_use] pub fn new () -> Self { Self :: with_hasher (RandomState :: new ()) } # [doc = " Creates an empty `JoinMap` with the specified capacity."] # [doc = ""] # [doc = " The `JoinMap` will be able to hold at least `capacity` tasks without"] # [doc = " reallocating."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tokio_util::task::JoinMap;"] # [doc = " let map: JoinMap<&str, i32> = JoinMap::with_capacity(10);"] # [doc = " ```"] # [inline] # [must_use] pub fn with_capacity (capacity : usize) -> Self { JoinMap :: with_capacity_and_hasher (capacity , Default :: default ()) } }
    };
}

impl_17!()