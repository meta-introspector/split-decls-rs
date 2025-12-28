macro_rules! deps {
    () => {
        DefaultSpawn!();
        ThreadPoolBuilder!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl Default for ThreadPoolBuilder { fn default () -> Self { ThreadPoolBuilder { num_threads : 0 , panic_handler : None , get_thread_name : None , stack_size : None , start_handler : None , exit_handler : None , deadlock_handler : None , acquire_thread_handler : None , release_thread_handler : None , spawn_handler : DefaultSpawn , breadth_first : false , } } }
    };
}

impl_334!();