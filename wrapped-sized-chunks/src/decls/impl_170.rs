macro_rules! deps {
    () => {
        Slice!();
        RingBuffer!();
        Iter!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < 'a , A : 'a , const N : usize > Slice < 'a , A , N > { # [doc = " Get an unchecked reference to the value at the given index."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " You must ensure the index is not out of bounds."] # [must_use] pub unsafe fn get_unchecked (& self , index : usize) -> & A { self . buffer . get_unchecked (self . range . start + index) } # [doc = " Get an iterator over references to the items in the slice in order."] # [inline] # [must_use] pub fn iter (& self) -> Iter < '_ , A , N > { Iter { buffer : self . buffer , left_index : self . buffer . origin + self . range . start , right_index : self . buffer . origin + self . range . start + self . len () , remaining : self . len () , } } # [doc = " Create a subslice of this slice."] # [doc = ""] # [doc = " This consumes the slice. To create a subslice without consuming it,"] # [doc = " clone it first: `my_slice.clone().slice(1..2)`."] # [must_use] pub fn slice < R : RangeBounds < usize > > (self , range : R) -> Slice < 'a , A , N > { let new_range = Range { start : match range . start_bound () { Bound :: Unbounded => self . range . start , Bound :: Included (index) => self . range . start + index , Bound :: Excluded (_) => unimplemented ! () , } , end : match range . end_bound () { Bound :: Unbounded => self . range . end , Bound :: Included (index) => self . range . start + index + 1 , Bound :: Excluded (index) => self . range . start + index , } , } ; if new_range . start < self . range . start || new_range . end > self . range . end || new_range . start > new_range . end { panic ! ("Slice::slice: index out of bounds") ; } Slice { buffer : self . buffer , range : new_range , } } # [doc = " Split the slice into two subslices at the given index."] # [must_use] pub fn split_at (self , index : usize) -> (Slice < 'a , A , N > , Slice < 'a , A , N >) { if index > self . len () { panic ! ("Slice::split_at: index out of bounds") ; } let index = self . range . start + index ; (Slice { buffer : self . buffer , range : Range { start : self . range . start , end : index , } , } , Slice { buffer : self . buffer , range : Range { start : index , end : self . range . end , } , } ,) } # [doc = " Construct a new `RingBuffer` by copying the elements in this slice."] # [inline] # [must_use] pub fn to_owned (& self) -> RingBuffer < A , N > where A : Clone , { self . iter () . cloned () . collect () } }
    };
}

impl_170!()