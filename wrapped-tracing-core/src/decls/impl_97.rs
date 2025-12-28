macro_rules! deps {
    () => {
        Entered!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Drop for Entered < '_ > { # [inline] fn drop (& mut self) { self . 0 . can_enter . set (true) ; } }
    };
}

impl_97!()