macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Drop for Env < '_ > { fn drop (& mut self) { for (var , prev_value) in self . altered_vars . iter () . rev () { match prev_value { Some (value) => env :: set_var (var , value) , None => env :: remove_var (var) , } } } }
    };
}

impl_59!()