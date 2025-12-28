macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        # [cfg (feature = "grab_spare_slice")] impl < 's , T > SliceVec < 's , T > { # [doc = " Obtain the shared slice of the array _after_ the active memory."] # [doc = ""] # [doc = " ## Example"] # [doc = " ```rust"] # [doc = " # use tinyvec::*;"] # [doc = " let mut arr = [0; 4];"] # [doc = " let mut sv = SliceVec::from_slice_len(&mut arr, 0);"] # [doc = " assert_eq!(sv.grab_spare_slice().len(), 4);"] # [doc = " sv.push(10);"] # [doc = " sv.push(11);"] # [doc = " sv.push(12);"] # [doc = " sv.push(13);"] # [doc = " assert_eq!(sv.grab_spare_slice().len(), 0);"] # [doc = " ```"] # [must_use] # [inline (always)] pub fn grab_spare_slice (& self) -> & [T] { & self . data [self . len ..] } # [doc = " Obtain the mutable slice of the array _after_ the active memory."] # [doc = ""] # [doc = " ## Example"] # [doc = " ```rust"] # [doc = " # use tinyvec::*;"] # [doc = " let mut arr = [0; 4];"] # [doc = " let mut sv = SliceVec::from_slice_len(&mut arr, 0);"] # [doc = " assert_eq!(sv.grab_spare_slice_mut().len(), 4);"] # [doc = " sv.push(10);"] # [doc = " sv.push(11);"] # [doc = " assert_eq!(sv.grab_spare_slice_mut().len(), 2);"] # [doc = " ```"] # [inline (always)] pub fn grab_spare_slice_mut (& mut self) -> & mut [T] { & mut self . data [self . len ..] } }
    };
}

impl_87!();