macro_rules! deps {
    () => {
        ProcSelector!();
        Result!();
    };
}

macro_rules! procctl_get_optional {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn procctl_get_optional < P > (option : c_int , process : ProcSelector ,) -> io :: Result < P > { let mut value : MaybeUninit < P > = MaybeUninit :: uninit () ; procctl (option , process , value . as_mut_ptr () . cast ()) ? ; Ok (value . assume_init ()) }
    };
}

procctl_get_optional!();