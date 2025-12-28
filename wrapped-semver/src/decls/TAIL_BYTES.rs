macro_rules! TAIL_BYTES {
    () => {
        const TAIL_BYTES : usize = 8 * (PTR_BYTES < 8) as usize - PTR_BYTES * (PTR_BYTES < 8) as usize ;
    };
}

TAIL_BYTES!()