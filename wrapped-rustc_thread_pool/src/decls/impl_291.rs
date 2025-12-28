macro_rules! deps {
    () => {
        Registry!();
        WorkerLocal!();
        WorkerThread!();
        CacheAligned!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < T > WorkerLocal < T > { # [doc = " Creates a new worker local where the `initial` closure computes the"] # [doc = " value this worker local should take for each thread in the thread pool."] # [inline] pub fn new < F : FnMut (usize) -> T > (mut initial : F) -> WorkerLocal < T > { let registry = Registry :: current () ; WorkerLocal { locals : (0 .. registry . num_threads ()) . map (| i | CacheAligned (initial (i))) . collect () , registry , } } # [doc = " Returns the worker-local value for each thread"] # [inline] pub fn into_inner (self) -> Vec < T > { self . locals . into_iter () . map (| c | c . 0) . collect () } fn current (& self) -> & T { unsafe { let worker_thread = WorkerThread :: current () ; if worker_thread . is_null () || & * (* worker_thread) . registry as * const _ != & * self . registry as * const _ { panic ! ("WorkerLocal can only be used on the thread pool it was created on") } & self . locals [(* worker_thread) . index] . 0 } } }
    };
}

impl_291!();