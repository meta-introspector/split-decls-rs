macro_rules! read_variable_length_byte_field {
    () => {
        # [inline] fn read_variable_length_byte_field < R : Read > (reader : & mut R , len : usize) -> io :: Result < Box < [u8] > > { let mut data = vec ! [0 ; len] . into_boxed_slice () ; reader . read_exact (& mut data) ? ; Ok (data) }
    };
}

read_variable_length_byte_field!();