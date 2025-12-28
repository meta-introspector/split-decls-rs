macro_rules! impl_1271 {
    () => {
        # [cfg (linux_kernel)] impl MembarrierQuery { # [doc = " Test whether this query result contains the given command."] # [inline] pub fn contains_command (self , cmd : MembarrierCommand) -> bool { self . contains (Self :: from_bits_retain (cmd as _)) } }
    };
}

impl_1271!();