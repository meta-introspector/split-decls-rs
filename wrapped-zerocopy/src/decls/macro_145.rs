macro_rules! macro_145 {
    () => {
        define_type ! (An , "A word-sized signed integer" , Isize , isize , mem :: size_of ::< isize > () * 8 , mem :: size_of ::< isize > () , isize :: from_be_bytes , isize :: to_be_bytes , isize :: from_le_bytes , isize :: to_le_bytes , "signed integer" , [] , [] , [] , []) ;
    };
}

macro_145!();