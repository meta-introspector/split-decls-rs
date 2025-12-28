macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! threaded_read {
    () => {
        deps!();
        fn threaded_read < R > (mut input : R) -> Stream where R : std :: io :: Read + Send + 'static , { std :: thread :: spawn (move | | { let mut ret = Vec :: new () ; input . read_to_end (& mut ret) . map (| _ | ret) }) }
    };
}

threaded_read!()