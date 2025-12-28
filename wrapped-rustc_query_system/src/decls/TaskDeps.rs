macro_rules! deps {
    () => {
        DepNode!();
        EdgesVec!();
    };
}

macro_rules! TaskDeps {
    () => {
        deps!();
        # [derive (Debug)] pub struct TaskDeps { # [cfg (debug_assertions)] node : Option < DepNode > , reads : EdgesVec , read_set : FxHashSet < DepNodeIndex > , phantom_data : PhantomData < DepNode > , }
    };
}

TaskDeps!();