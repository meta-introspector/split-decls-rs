macro_rules! deps {
    () => {
        Lanes!();
        Accumulators!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Accumulators { const fn new (seed : u32) -> Self { Self ([seed . wrapping_add (PRIME32_1) . wrapping_add (PRIME32_2) , seed . wrapping_add (PRIME32_2) , seed , seed . wrapping_sub (PRIME32_1) ,]) } # [inline] fn write (& mut self , lanes : Lanes) { let [acc1 , acc2 , acc3 , acc4] = & mut self . 0 ; let [lane1 , lane2 , lane3 , lane4] = lanes ; * acc1 = round (* acc1 , lane1 . to_le ()) ; * acc2 = round (* acc2 , lane2 . to_le ()) ; * acc3 = round (* acc3 , lane3 . to_le ()) ; * acc4 = round (* acc4 , lane4 . to_le ()) ; } # [inline] fn write_many < 'd > (& mut self , mut data : & 'd [u8]) -> & 'd [u8] { while let Some ((chunk , rest)) = data . split_first_chunk :: < BYTES_IN_LANE > () { let lanes = unsafe { chunk . as_ptr () . cast :: < Lanes > () . read_unaligned () } ; self . write (lanes) ; data = rest ; } data } # [inline] const fn finish (& self) -> u32 { let [acc1 , acc2 , acc3 , acc4] = self . 0 ; let acc1 = acc1 . rotate_left (1) ; let acc2 = acc2 . rotate_left (7) ; let acc3 = acc3 . rotate_left (12) ; let acc4 = acc4 . rotate_left (18) ; acc1 . wrapping_add (acc2) . wrapping_add (acc3) . wrapping_add (acc4) } }
    };
}

impl_17!();