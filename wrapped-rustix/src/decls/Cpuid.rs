macro_rules! Cpuid {
    () => {
        # [doc = " A Linux CPU ID."] # [cfg (linux_kernel)] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Debug , Hash)] pub struct Cpuid (RawCpuid) ;
    };
}

Cpuid!();