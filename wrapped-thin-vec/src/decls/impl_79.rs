macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'a , T > Drain < 'a , T > { # [doc = " Returns the remaining items of this iterator as a slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::thin_vec;"] # [doc = ""] # [doc = " let mut vec = thin_vec!['a', 'b', 'c'];"] # [doc = " let mut drain = vec.drain(..);"] # [doc = " assert_eq!(drain.as_slice(), &['a', 'b', 'c']);"] # [doc = " let _ = drain.next().unwrap();"] # [doc = " assert_eq!(drain.as_slice(), &['b', 'c']);"] # [doc = " ```"] # [must_use] pub fn as_slice (& self) -> & [T] { self . iter . as_slice () } }
    };
}

impl_79!()