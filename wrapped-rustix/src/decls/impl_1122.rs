macro_rules! deps {
    () => {
        WaitIdStatus!();
        Result!();
    };
}

macro_rules! impl_1122 {
    () => {
        deps!();
        # [cfg (not (any (target_os = "horizon" , target_os = "openbsd" , target_os = "redox" , target_os = "wasi")))] impl fmt :: Debug for WaitIdStatus { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("WaitIdStatus") ; s . field ("stopped" , & self . stopped ()) ; s . field ("exited" , & self . exited ()) ; s . field ("killed" , & self . killed ()) ; s . field ("trapped" , & self . trapped ()) ; s . field ("dumped" , & self . dumped ()) ; s . field ("continued" , & self . continued ()) ; # [cfg (not (any (target_os = "emscripten" , target_os = "fuchsia" , target_os = "netbsd")))] if let Some (stopping_signal) = self . stopping_signal () { s . field ("stopping_signal" , & stopping_signal) ; } # [cfg (not (any (target_os = "emscripten" , target_os = "fuchsia" , target_os = "netbsd")))] if let Some (trapping_signal) = self . trapping_signal () { s . field ("trapping_signal" , & trapping_signal) ; } # [cfg (not (any (target_os = "emscripten" , target_os = "fuchsia" , target_os = "netbsd")))] if let Some (exit_status) = self . exit_status () { s . field ("exit_status" , & exit_status) ; } # [cfg (not (any (target_os = "emscripten" , target_os = "fuchsia" , target_os = "netbsd")))] if let Some (terminating_signal) = self . terminating_signal () { s . field ("terminating_signal" , & terminating_signal) ; } s . finish () } }
    };
}

impl_1122!();