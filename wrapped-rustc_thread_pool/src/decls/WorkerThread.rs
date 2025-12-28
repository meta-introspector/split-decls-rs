macro_rules! deps {
    () => {
        JobFifo!();
        XorShift64Star!();
        JobRef!();
        Registry!();
    };
}

macro_rules! WorkerThread {
    () => {
        deps!();
        # [doc = " ////////////////////////////////////////////////////////////////////////"] # [doc = " WorkerThread identifiers"] pub (super) struct WorkerThread { # [doc = " the \"worker\" half of our local deque"] worker : Worker < JobRef > , # [doc = " the \"stealer\" half of the worker's broadcast deque"] stealer : Stealer < JobRef > , # [doc = " local queue used for `spawn_fifo` indirection"] fifo : JobFifo , pub (crate) index : usize , # [doc = " A weak random number generator."] rng : XorShift64Star , pub (crate) registry : Arc < Registry > , }
    };
}

WorkerThread!();