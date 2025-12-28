macro_rules! stripes_with_tail {
    () => {
        # [inline] pub fn stripes_with_tail (block : & [u8]) -> (& [[u8 ; 64]] , & [u8]) { match block . bp_as_chunks () { ([stripes @ .. , last] , []) => (stripes , last) , (stripes , last) => (stripes , last) , } }
    };
}

stripes_with_tail!()