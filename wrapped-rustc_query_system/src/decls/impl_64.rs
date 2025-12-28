macro_rules! deps {
    () => {
        TaskDeps!();
        EdgesVec!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Default for TaskDeps { fn default () -> Self { Self { # [cfg (debug_assertions)] node : None , reads : EdgesVec :: new () , read_set : FxHashSet :: with_capacity_and_hasher (128 , Default :: default ()) , phantom_data : PhantomData , } } }
    };
}

impl_64!()