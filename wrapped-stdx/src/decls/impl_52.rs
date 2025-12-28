macro_rules! deps {
    () => {
        JoinHandle!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T > JoinHandle < T > { # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if there is no thread to join."] # [must_use] pub fn join (mut self) -> T { self . inner . take () . unwrap () . join () } }
    };
}

impl_52!();