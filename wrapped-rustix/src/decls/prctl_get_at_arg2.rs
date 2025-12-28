macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! prctl_get_at_arg2 {
    () => {
        deps!();
        # [inline] pub (crate) unsafe fn prctl_get_at_arg2 < P , T > (option : i32) -> io :: Result < T > where P : Default , T : TryFrom < P , Error = io :: Errno > , { let mut value : P = Default :: default () ; prctl_2args (option , as_mut_ptr (& mut value) . cast ()) ? ; TryFrom :: try_from (value) }
    };
}

prctl_get_at_arg2!();