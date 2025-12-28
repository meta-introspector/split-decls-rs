macro_rules! deps {
    () => {
        ParseBuffer!();
        Unexpected!();
    };
}

macro_rules! get_unexpected {
    () => {
        deps!();
        pub (crate) fn get_unexpected (buffer : & ParseBuffer) -> Rc < Cell < Unexpected > > { cell_clone (& buffer . unexpected) . unwrap () }
    };
}

get_unexpected!();