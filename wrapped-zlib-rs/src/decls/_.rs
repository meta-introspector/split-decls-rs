macro_rules! _ {
    () => {
        const _ : () = assert ! (ALIGN as usize % mem :: size_of ::<* mut c_void > () == 0) ;
    };
}

_!()