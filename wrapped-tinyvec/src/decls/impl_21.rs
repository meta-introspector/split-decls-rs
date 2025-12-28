macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [cfg (feature = "grab_spare_slice")] impl < A : Array > ArrayVec < A > { # [doc = " Obtain the shared slice of the array _after_ the active memory."] # [doc = ""] # [doc = " ## Example"] # [doc = " ```rust"] # [doc = " # use tinyvec::*;"] # [doc = " let mut av = array_vec!([i32; 4]);"] # [doc = " assert_eq!(av.grab_spare_slice().len(), 4);"] # [doc = " av.push(10);"] # [doc = " av.push(11);"] # [doc = " av.push(12);"] # [doc = " av.push(13);"] # [doc = " assert_eq!(av.grab_spare_slice().len(), 0);"] # [doc = " ```"] # [inline (always)] pub fn grab_spare_slice (& self) -> & [A :: Item] { & self . data . as_slice () [self . len as usize ..] } # [doc = " Obtain the mutable slice of the array _after_ the active memory."] # [doc = ""] # [doc = " ## Example"] # [doc = " ```rust"] # [doc = " # use tinyvec::*;"] # [doc = " let mut av = array_vec!([i32; 4]);"] # [doc = " assert_eq!(av.grab_spare_slice_mut().len(), 4);"] # [doc = " av.push(10);"] # [doc = " av.push(11);"] # [doc = " assert_eq!(av.grab_spare_slice_mut().len(), 2);"] # [doc = " ```"] # [inline (always)] pub fn grab_spare_slice_mut (& mut self) -> & mut [A :: Item] { & mut self . data . as_slice_mut () [self . len as usize ..] } }
    };
}

impl_21!();