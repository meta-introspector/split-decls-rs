macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! macro_54 {
    () => {
        deps!();
        impl_stream_forward ! ({ impl <'sval , 'a , S : ? Sized > Stream <'sval > for &'a mut S where S : Stream <'sval > } => x => { ** x }) ;
    };
}

macro_54!()