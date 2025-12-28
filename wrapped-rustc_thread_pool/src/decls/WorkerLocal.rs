macro_rules! deps {
    () => {
        CacheAligned!();
        Registry!();
    };
}

macro_rules! WorkerLocal {
    () => {
        deps!();
        # [doc = " Holds worker-locals values for each thread in a thread pool."] # [doc = " You can only access the worker local value through the Deref impl"] # [doc = " on the thread pool it was constructed on. It will panic otherwise"] pub struct WorkerLocal < T > { locals : Vec < CacheAligned < T > > , registry : Arc < Registry > , }
    };
}

WorkerLocal!();