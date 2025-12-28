macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
        ThreadPoolBuildError!();
        Registry!();
        WorkerThread!();
        Latch!();
    };
}

macro_rules! default_global_registry {
    () => {
        deps!();
        fn default_global_registry () -> Result < Arc < Registry > , ThreadPoolBuildError > { let result = Registry :: new (ThreadPoolBuilder :: new ()) ; let unsupported = matches ! (& result , Err (e) if e . is_unsupported ()) ; if unsupported && WorkerThread :: current () . is_null () { let builder = ThreadPoolBuilder :: new () . num_threads (1) . spawn_handler (| thread | { let worker_thread = Box :: leak (Box :: new (WorkerThread :: from (thread))) ; let registry = & * worker_thread . registry ; let index = worker_thread . index ; unsafe { WorkerThread :: set_current (worker_thread) ; Latch :: set (& registry . thread_infos [index] . primed) ; } Ok (()) }) ; let fallback_result = Registry :: new (builder) ; if fallback_result . is_ok () { return fallback_result ; } } result }
    };
}

default_global_registry!();