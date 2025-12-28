macro_rules! deps {
    () => {
        JobResult!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T > JobResult < T > { fn call (func : impl FnOnce (bool) -> T) -> Self { match unwind :: halt_unwinding (| | func (true)) { Ok (x) => JobResult :: Ok (x) , Err (x) => JobResult :: Panic (x) , } } # [doc = " Convert the `JobResult` for a job that has finished (and hence"] # [doc = " its JobResult is populated) into its return value."] # [doc = ""] # [doc = " NB. This will panic if the job panicked."] pub (super) fn into_return_value (self) -> T { match self { JobResult :: None => unreachable ! () , JobResult :: Ok (x) => x , JobResult :: Panic (x) => unwind :: resume_unwinding (x) , } } }
    };
}

impl_46!();