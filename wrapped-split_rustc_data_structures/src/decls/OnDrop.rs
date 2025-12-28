macro_rules! OnDrop {
    () => {
        pub struct OnDrop < F : FnOnce () > (Option < F >) ;
    };
}

OnDrop!()