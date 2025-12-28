macro_rules! deps {
    () => {
        Entered!();
        Dispatch!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a > Entered < 'a > { # [inline] fn current (& self) -> Ref < 'a , Dispatch > { let default = self . 0 . default . borrow () ; Ref :: map (default , | default | match default { Some (default) => default , None => get_global () , }) } }
    };
}

impl_96!();