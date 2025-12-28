macro_rules! deps {
    () => {
        Registry!();
        ThreadPoolBuildError!();
        ThreadPoolBuilder!();
        ThreadSpawn!();
    };
}

macro_rules! init_global_registry {
    () => {
        deps!();
        # [doc = " Starts the worker threads (if that has not already happened) with"] # [doc = " the given builder."] pub (super) fn init_global_registry < S > (builder : ThreadPoolBuilder < S > ,) -> Result < & 'static Arc < Registry > , ThreadPoolBuildError > where S : ThreadSpawn , { set_global_registry (| | Registry :: new (builder)) }
    };
}

init_global_registry!()