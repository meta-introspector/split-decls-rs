macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < T , const N : usize > TryFrom < ThinVec < T > > for [T ; N] { type Error = ThinVec < T > ; # [doc = " Gets the entire contents of the `ThinVec<T>` as an array,"] # [doc = " if its size exactly matches that of the requested array."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = " use std::convert::TryInto;"] # [doc = ""] # [doc = " assert_eq!(thin_vec![1, 2, 3].try_into(), Ok([1, 2, 3]));"] # [doc = " assert_eq!(<ThinVec<i32>>::new().try_into(), Ok([]));"] # [doc = " ```"] # [doc = ""] # [doc = " If the length doesn't match, the input comes back in `Err`:"] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = " use std::convert::TryInto;"] # [doc = ""] # [doc = " let r: Result<[i32; 4], _> = (0..10).collect::<ThinVec<_>>().try_into();"] # [doc = " assert_eq!(r, Err(thin_vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));"] # [doc = " ```"] # [doc = ""] # [doc = " If you're fine with just getting a prefix of the `ThinVec<T>`,"] # [doc = " you can call [`.truncate(N)`](ThinVec::truncate) first."] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = " use std::convert::TryInto;"] # [doc = ""] # [doc = " let mut v = ThinVec::from(\"hello world\");"] # [doc = " v.sort();"] # [doc = " v.truncate(2);"] # [doc = " let [a, b]: [_; 2] = v.try_into().unwrap();"] # [doc = " assert_eq!(a, b' ');"] # [doc = " assert_eq!(b, b'd');"] # [doc = " ```"] fn try_from (mut vec : ThinVec < T >) -> Result < [T ; N] , ThinVec < T > > { if vec . len () != N { return Err (vec) ; } unsafe { vec . set_len (0) } ; let array = unsafe { ptr :: read (vec . data_raw () as * const [T ; N]) } ; Ok (array) } }
    };
}

impl_59!();