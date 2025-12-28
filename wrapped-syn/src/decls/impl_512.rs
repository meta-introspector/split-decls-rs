macro_rules! deps {
    () => {
        Unexpected!();
    };
}

macro_rules! impl_512 {
    () => {
        deps!();
        impl Clone for Unexpected { fn clone (& self) -> Self { match self { Unexpected :: None => Unexpected :: None , Unexpected :: Some (span , delimiter) => Unexpected :: Some (* span , * delimiter) , Unexpected :: Chain (next) => Unexpected :: Chain (next . clone ()) , } } }
    };
}

impl_512!()