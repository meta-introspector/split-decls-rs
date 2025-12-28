macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Reset for Sha1 { # [inline] fn reset (& mut self) { self . h = INITIAL_H ; self . block_len = 0 ; self . buffer . reset () ; if let Some (ref mut ctx) = self . detection { ctx . reset () ; } } }
    };
}

impl_12!()