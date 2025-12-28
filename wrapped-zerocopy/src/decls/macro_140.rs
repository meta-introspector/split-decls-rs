macro_rules! macro_140 {
    () => {
        define_type ! (A , "A word-sized unsigned integer" , Usize , usize , mem :: size_of ::< usize > () * 8 , mem :: size_of ::< usize > () , usize :: from_be_bytes , usize :: to_be_bytes , usize :: from_le_bytes , usize :: to_le_bytes , "unsigned integer" , [] , [] , [] , []) ;
    };
}

macro_140!();