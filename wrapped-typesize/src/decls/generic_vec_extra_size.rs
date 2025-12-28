macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! generic_vec_extra_size {
    () => {
        deps!();
        pub (crate) fn generic_vec_extra_size < 'a , T : TypeSize + 'a > (iter : impl Iterator < Item = & 'a T > , capacity : usize , len : usize ,) -> usize { iter . map (TypeSize :: get_size) . sum :: < usize > () + (capacity - len) * core :: mem :: size_of :: < T > () }
    };
}

generic_vec_extra_size!();