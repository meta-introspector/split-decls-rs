macro_rules! RecvFlags {
    () => {
        # [doc = " Flags for incoming messages."] # [doc = ""] # [doc = " Flags provide additional information about incoming messages."] # [cfg (not (target_os = "redox"))] # [derive (Copy , Clone , Eq , PartialEq)] pub struct RecvFlags (c_int) ;
    };
}

RecvFlags!()