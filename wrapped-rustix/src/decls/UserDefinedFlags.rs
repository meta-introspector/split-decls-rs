macro_rules! UserDefinedFlags {
    () => {
        # [doc = " User-defined flags."] # [doc = ""] # [doc = " Only the lower 24 bits are used in this struct."] # [repr (transparent)] # [cfg (any (apple , freebsdlike))] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub struct UserDefinedFlags (u32) ;
    };
}

UserDefinedFlags!();