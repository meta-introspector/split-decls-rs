macro_rules! deps {
    () => {
        Decompositions!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < I > Decompositions < I > { # [inline] fn push_back (& mut self , ch : char) { let class = super :: char :: canonical_combining_class (ch) ; if class == 0 { self . sort_pending () ; self . buffer . push ((class , ch)) ; self . ready . end = self . buffer . len () ; } else { self . buffer . push ((class , ch)) ; } } # [inline] fn sort_pending (& mut self) { self . buffer [self . ready . end ..] . sort_by_key (| k | k . 0) ; } # [inline] fn reset_buffer (& mut self) { let pending = self . buffer . len () - self . ready . end ; for i in 0 .. pending { self . buffer [i] = self . buffer [i + self . ready . end] ; } self . buffer . truncate (pending) ; self . ready = 0 .. 0 ; } # [inline] fn increment_next_ready (& mut self) { let next = self . ready . start + 1 ; if next == self . ready . end { self . reset_buffer () ; } else { self . ready . start = next ; } } }
    };
}

impl_6!()