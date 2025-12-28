macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T : Clone > ThinVec < T > { # [doc = " Resizes the `Vec` in-place so that `len()` is equal to `new_len`."] # [doc = ""] # [doc = " If `new_len` is greater than `len()`, the `Vec` is extended by the"] # [doc = " difference, with each additional slot filled with `value`."] # [doc = " If `new_len` is less than `len()`, the `Vec` is simply truncated."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [cfg_attr (not (feature = "gecko-ffi") , doc = "```")] # [cfg_attr (feature = "gecko-ffi" , doc = "```ignore")] # [doc = " # #[macro_use] extern crate thin_vec;"] # [doc = " # fn main() {"] # [doc = " let mut vec = thin_vec![\"hello\"];"] # [doc = " vec.resize(3, \"world\");"] # [doc = " assert_eq!(vec, [\"hello\", \"world\", \"world\"]);"] # [doc = ""] # [doc = " let mut vec = thin_vec![1, 2, 3, 4];"] # [doc = " vec.resize(2, 0);"] # [doc = " assert_eq!(vec, [1, 2]);"] # [doc = " # }"] # [doc = " ```"] pub fn resize (& mut self , new_len : usize , value : T) { let old_len = self . len () ; if new_len > old_len { let additional = new_len - old_len ; self . reserve (additional) ; for _ in 1 .. additional { self . push (value . clone ()) ; } if additional > 0 { self . push (value) ; } } else if new_len < old_len { self . truncate (new_len) ; } } # [doc = " Clones and appends all elements in a slice to the `ThinVec`."] # [doc = ""] # [doc = " Iterates over the slice `other`, clones each element, and then appends"] # [doc = " it to this `ThinVec`. The `other` slice is traversed in-order."] # [doc = ""] # [doc = " Note that this function is same as [`extend`] except that it is"] # [doc = " specialized to work with slices instead. If and when Rust gets"] # [doc = " specialization this function will likely be deprecated (but still"] # [doc = " available)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::thin_vec;"] # [doc = ""] # [doc = " let mut vec = thin_vec![1];"] # [doc = " vec.extend_from_slice(&[2, 3, 4]);"] # [doc = " assert_eq!(vec, [1, 2, 3, 4]);"] # [doc = " ```"] # [doc = ""] # [doc = " [`extend`]: ThinVec::extend"] pub fn extend_from_slice (& mut self , other : & [T]) { self . extend (other . iter () . cloned ()) } }
    };
}

impl_21!();