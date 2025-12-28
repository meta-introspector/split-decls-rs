macro_rules! deps {
    () => {
        JobFifo!();
        XorShift64Star!();
        ThreadBuilder!();
        WorkerThread!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl From < ThreadBuilder > for WorkerThread { fn from (thread : ThreadBuilder) -> Self { Self { worker : thread . worker , stealer : thread . stealer , fifo : JobFifo :: new () , index : thread . index , rng : XorShift64Star :: new () , registry : thread . registry , } } }
    };
}

impl_124!()