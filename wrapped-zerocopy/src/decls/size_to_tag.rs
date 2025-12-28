macro_rules! deps {
    () => {
        SizeToTag!();
    };
}

macro_rules! size_to_tag {
    () => {
        deps!();
        mod size_to_tag { pub trait SizeToTag < const SIZE : usize > { type Tag ; } impl SizeToTag < 1 > for () { type Tag = u8 ; } impl SizeToTag < 2 > for () { type Tag = u16 ; } impl SizeToTag < 4 > for () { type Tag = u32 ; } impl SizeToTag < 8 > for () { type Tag = u64 ; } impl SizeToTag < 16 > for () { type Tag = u128 ; } }
    };
}

size_to_tag!();