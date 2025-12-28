macro_rules! JoinHandle {
    () => {
        pub struct JoinHandle < T = () > { inner : Option < jod_thread :: JoinHandle < T > > , allow_leak : bool , }
    };
}

JoinHandle!();