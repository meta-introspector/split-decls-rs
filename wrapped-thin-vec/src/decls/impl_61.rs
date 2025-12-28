macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T > IntoIter < T > { # [doc = " Returns the remaining items of this iterator as a slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::thin_vec;"] # [doc = ""] # [doc = " let vec = thin_vec!['a', 'b', 'c'];"] # [doc = " let mut into_iter = vec.into_iter();"] # [doc = " assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);"] # [doc = " let _ = into_iter.next().unwrap();"] # [doc = " assert_eq!(into_iter.as_slice(), &['b', 'c']);"] # [doc = " ```"] pub fn as_slice (& self) -> & [T] { unsafe { slice :: from_raw_parts (self . vec . data_raw () . add (self . start) , self . len ()) } } # [doc = " Returns the remaining items of this iterator as a mutable slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::thin_vec;"] # [doc = ""] # [doc = " let vec = thin_vec!['a', 'b', 'c'];"] # [doc = " let mut into_iter = vec.into_iter();"] # [doc = " assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);"] # [doc = " into_iter.as_mut_slice()[2] = 'z';"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'a');"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'b');"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'z');"] # [doc = " ```"] pub fn as_mut_slice (& mut self) -> & mut [T] { unsafe { & mut * self . as_raw_mut_slice () } } fn as_raw_mut_slice (& mut self) -> * mut [T] { unsafe { ptr :: slice_from_raw_parts_mut (self . vec . data_raw () . add (self . start) , self . len ()) } } }
    };
}

impl_61!();