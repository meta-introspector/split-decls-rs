macro_rules! MAX_EIGHTBYTES {
    () => {
        const MAX_EIGHTBYTES : usize = LARGEST_VECTOR_SIZE / 64 ;
    };
}

MAX_EIGHTBYTES!()