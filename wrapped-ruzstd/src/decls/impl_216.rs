macro_rules! deps {
    () => {
        Read!();
        Reservoir!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl Reservoir { # [doc = " Initialize a new empty reservoir, creating an allocation of `size`."] pub fn new (size : usize) -> Self { assert ! (size >= 16 , "Reservoirs cannot be below 16 bytes in size") ; let lake : Vec < u8 > = vec ! [0 ; size] ; let k = K as u16 ; Self { lake , k } } # [doc = " Filling the reservoir is performed using Algorithm L."] # [doc = ""] # [doc = " The return value is the populated reservoir."] pub fn fill < R : io :: Read > (mut self , source : & mut R) -> Vec < u8 > { let mut total_bytes_read : usize = 0 ; while let Ok (num_bytes) = source . read (self . lake . as_mut_slice ()) { total_bytes_read += num_bytes ; if total_bytes_read == self . lake . len () { break ; } if num_bytes == 0 { self . lake . resize (total_bytes_read , 0) ; } } let mut threshold = E . powf (fastrand :: f64 () . ln () / f64 :: from (self . k)) ; let mut next = self . lake . len () ; let mut lake_chunks = self . lake . chunks_mut (self . k as usize) . collect :: < Vec < & mut [u8] > > () ; let end_of_lake = lake_chunks . len () ; let mut counter = end_of_lake / self . k as usize ; let mut dumpster = Vec :: with_capacity (self . k as usize) ; loop { let num_bytes_read : u64 ; if counter == next { num_bytes_read = source . read (lake_chunks [fastrand :: usize (0 .. end_of_lake)]) . unwrap () as u64 ; next += ((fastrand :: f64 () . ln () / f64 :: ln (1.0 - threshold)) . floor () as usize + 1) * self . k as usize ; threshold *= E . powf (fastrand :: f64 () . ln () / f64 :: from (end_of_lake as u32)) } else { num_bytes_read = source . read (& mut dumpster) . unwrap () as u64 ; } if num_bytes_read == 0 { break ; } counter += self . k as usize ; } self . lake . shrink_to_fit () ; self . lake } }
    };
}

impl_216!();