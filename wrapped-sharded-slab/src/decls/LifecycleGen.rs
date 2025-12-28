macro_rules! deps {
    () => {
        Generation!();
    };
}

macro_rules! LifecycleGen {
    () => {
        deps!();
        struct LifecycleGen < C > (Generation < C >) ;
    };
}

LifecycleGen!()