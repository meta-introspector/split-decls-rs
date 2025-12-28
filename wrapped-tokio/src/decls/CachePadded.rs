macro_rules! CachePadded {
    () => {
        # [doc = " Pads and aligns a value to the length of a cache line."] # [derive (Clone , Copy , Default , Hash , PartialEq , Eq)] # [cfg_attr (any (target_arch = "x86_64" , target_arch = "aarch64" , target_arch = "powerpc64" ,) , repr (align (128)))] # [cfg_attr (any (target_arch = "arm" , target_arch = "mips" , target_arch = "mips64" ,) , repr (align (32)))] # [cfg_attr (target_arch = "s390x" , repr (align (256)))] # [cfg_attr (not (any (target_arch = "x86_64" , target_arch = "aarch64" , target_arch = "powerpc64" , target_arch = "arm" , target_arch = "mips" , target_arch = "mips64" , target_arch = "s390x" ,)) , repr (align (64)))] pub (crate) struct CachePadded < T > { value : T , }
    };
}

CachePadded!();