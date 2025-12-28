macro_rules! deps {
    () => {
        Pid!();
    };
}

macro_rules! PTracer {
    () => {
        deps!();
        # [doc = " Process ptracer."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum PTracer { # [doc = " None."] None , # [doc = " Disable `ptrace` restrictions for the calling process."] Any , # [doc = " Specific process."] ProcessID (Pid) , }
    };
}

PTracer!();