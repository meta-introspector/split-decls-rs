macro_rules! deps {
    () => {
        ThreadPoolBuildError!();
        ThreadPoolBuilder!();
        ThreadPool!();
        ThreadSpawn!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        # [doc = " Note: the `S: ThreadSpawn` constraint is an internal implementation detail for the"] # [doc = " default spawn and those set by [`spawn_handler`](#method.spawn_handler)."] impl < S > ThreadPoolBuilder < S > where S : ThreadSpawn , { # [doc = " Creates a new `ThreadPool` initialized using this configuration."] pub fn build (self) -> Result < ThreadPool , ThreadPoolBuildError > { ThreadPool :: build (self) } # [doc = " Initializes the global thread pool. This initialization is"] # [doc = " **optional**. If you do not call this function, the thread pool"] # [doc = " will be automatically initialized with the default"] # [doc = " configuration. Calling `build_global` is not recommended, except"] # [doc = " in two scenarios:"] # [doc = ""] # [doc = " - You wish to change the default configuration."] # [doc = " - You are running a benchmark, in which case initializing may"] # [doc = "   yield slightly more consistent results, since the worker threads"] # [doc = "   will already be ready to go even in the first iteration. But"] # [doc = "   this cost is minimal."] # [doc = ""] # [doc = " Initialization of the global thread pool happens exactly"] # [doc = " once. Once started, the configuration cannot be"] # [doc = " changed. Therefore, if you call `build_global` a second time, it"] # [doc = " will return an error. An `Ok` result indicates that this"] # [doc = " is the first initialization of the thread pool."] pub fn build_global (self) -> Result < () , ThreadPoolBuildError > { let registry = registry :: init_global_registry (self) ? ; registry . wait_until_primed () ; Ok (()) } }
    };
}

impl_338!();