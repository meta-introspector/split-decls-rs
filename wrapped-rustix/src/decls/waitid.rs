macro_rules! deps {
    () => {
        WaitId!();
        Result!();
        WaitIdStatus!();
    };
}

macro_rules! waitid {
    () => {
        deps!();
        # [doc = " `waitid(_, _, _, opts)`—Wait for the specified child process to change"] # [doc = " state."] # [cfg (not (any (target_os = "cygwin" , target_os = "horizon" , target_os = "openbsd" , target_os = "redox" , target_os = "wasi" ,)))] # [inline] pub fn waitid < 'a , Id : Into < WaitId < 'a > > > (id : Id , options : WaitIdOptions ,) -> io :: Result < Option < WaitIdStatus > > { backend :: process :: syscalls :: waitid (id . into () , options) }
    };
}

waitid!();