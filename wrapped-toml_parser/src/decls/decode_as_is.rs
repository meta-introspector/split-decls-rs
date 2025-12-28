macro_rules! deps {
    () => {
        Raw!();
        ErrorSink!();
        StringBuilder!();
        ScalarKind!();
    };
}

macro_rules! decode_as_is {
    () => {
        deps!();
        pub (crate) fn decode_as_is < 'i > (raw : Raw < 'i > , kind : ScalarKind , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { let kind = decode_as (raw , raw . as_str () , kind , output , error) ; kind }
    };
}

decode_as_is!()