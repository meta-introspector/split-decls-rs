// Generated macro for impl_1255 (impl)
macro_rules! Depcrate_ioimpl_1255 {
() => {
// Module: crate::io
// Provides: {"impl_1255"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Read > Read for Take < T > { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { if self . limit == 0 { return Ok (0) ; } let max = cmp :: min (buf . len () as u64 , self . limit) as usize ; let n = self . inner . read (& mut buf [.. max]) ? ; assert ! (n as u64 <= self . limit , "number of read bytes exceeds limit") ; self . limit -= n as u64 ; Ok (n) } fn read_buf (& mut self , mut buf : BorrowedCursor < '_ >) -> Result < () > { if self . limit == 0 { return Ok (()) ; } if self . limit < buf . capacity () as u64 { let limit = self . limit as usize ; let extra_init = cmp :: min (limit , buf . init_mut () . len ()) ; let ibuf = unsafe { & mut buf . as_mut () [.. limit] } ; let mut sliced_buf : BorrowedBuf < '_ > = ibuf . into () ; unsafe { sliced_buf . set_init (extra_init) ; } let mut cursor = sliced_buf . unfilled () ; let result = self . inner . read_buf (cursor . reborrow ()) ; let new_init = cursor . init_mut () . len () ; let filled = sliced_buf . len () ; unsafe { buf . advance_unchecked (filled) ; buf . set_init (new_init) ; } self . limit -= filled as u64 ; result } else { let written = buf . written () ; let result = self . inner . read_buf (buf . reborrow ()) ; self . limit -= (buf . written () - written) as u64 ; result } } }
};
}
