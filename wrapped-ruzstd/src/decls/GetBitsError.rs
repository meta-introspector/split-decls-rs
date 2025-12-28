macro_rules! GetBitsError {
    () => {
        # [derive (Debug)] # [non_exhaustive] pub enum GetBitsError { TooManyBits { num_requested_bits : usize , limit : u8 , } , NotEnoughRemainingBits { requested : usize , remaining : usize , } , }
    };
}

GetBitsError!()