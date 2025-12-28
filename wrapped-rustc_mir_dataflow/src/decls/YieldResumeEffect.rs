macro_rules! YieldResumeEffect {
    () => {
        struct YieldResumeEffect < 'a > (& 'a mut DenseBitSet < Local >) ;
    };
}

YieldResumeEffect!();