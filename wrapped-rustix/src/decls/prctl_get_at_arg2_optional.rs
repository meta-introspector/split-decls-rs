macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! prctl_get_at_arg2_optional {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn prctl_get_at_arg2_optional < P > (option : i32) -> io :: Result < P > { let mut value : MaybeUninit < P > = MaybeUninit :: uninit () ; prctl_2args (option , value . as_mut_ptr () . cast ()) ? ; Ok (value . assume_init ()) }
    };
}

prctl_get_at_arg2_optional!()