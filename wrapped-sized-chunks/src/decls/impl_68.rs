macro_rules! deps {
    () => {
        Chunk!();
        InlineArray!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'a , A , T , const N : usize > From < & 'a mut InlineArray < A , T > > for Chunk < A , N > { fn from (array : & mut InlineArray < A , T >) -> Self { assert ! (InlineArray ::< A , T >:: CAPACITY <= Self :: CAPACITY || array . len () <= Self :: CAPACITY , "CAPACITY too small") ; let mut out = Self :: new () ; out . left = 0 ; out . right = array . len () ; unsafe { ptr :: copy_nonoverlapping (array . data () , out . mut_ptr (0) , out . right) ; * array . len_mut () = 0 ; } out } }
    };
}

impl_68!()