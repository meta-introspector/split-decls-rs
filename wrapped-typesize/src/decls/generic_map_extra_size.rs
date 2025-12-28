macro_rules! deps {
    () => {
        TypeSize!();
        EntryRef!();
    };
}

macro_rules! generic_map_extra_size {
    () => {
        deps!();
        pub (crate) fn generic_map_extra_size < 'a , K : TypeSize + 'a , V : TypeSize + 'a > (elements : impl Iterator < Item = impl EntryRef < K , V > > , capacity : usize , length : usize ,) -> usize { let element_size : usize = elements . map (| p | { let (key , value) = p . get_ref () ; key . get_size () + value . get_size () }) . sum () ; let free_space = capacity - length ; let free_size = free_space * (core :: mem :: size_of :: < K > () + core :: mem :: size_of :: < V > ()) ; element_size + free_size }
    };
}

generic_map_extra_size!();