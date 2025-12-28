macro_rules! fs {
    () => {
        # [cfg (not (windows))] # [cfg (not (feature = "fs"))] # [cfg (all (linux_raw , not (feature = "use-libc-auxv") , not (feature = "use-explicitly-provided-auxv") , any (feature = "param" , feature = "runtime" , feature = "thread" , feature = "time" , target_arch = "x86" ,)))] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] pub (crate) mod fs ;
    };
}

fs!();