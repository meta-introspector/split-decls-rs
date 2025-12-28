macro_rules! deps {
    () => {
        JoinHandle!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < T > Drop for JoinHandle < T > { fn drop (& mut self) { if ! self . allow_leak { return ; } if let Some (join_handle) = self . inner . take () { join_handle . detach () ; } } }
    };
}

impl_53!()