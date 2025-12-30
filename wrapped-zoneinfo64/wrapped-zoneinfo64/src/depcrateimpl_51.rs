// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl Debug for TzZoneData < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "TzZoneData {{ ") ? ; fn dbg_timestamp (f : & mut std :: fmt :: Formatter < '_ > , t : i64) -> std :: fmt :: Result { # [cfg (feature = "chrono")] let t = chrono :: DateTime :: from_timestamp (t , 0) . unwrap () ; write ! (f , "{t:?}, ") } write ! (f , "transitions/offsets: [") ? ; let (std , rule) = self . type_offsets [0] ; write ! (f , "{:?}, " , (std as f64 / 3600.0 , rule as f64 / 3600.0)) ? ; let mut i = 0 ; for & (hi , lo) in self . trans_pre32 { dbg_timestamp (f , ((hi as u32 as u64) << 32 | (lo as u32 as u64)) as i64) ? ; let (std , rule) = self . type_offsets [self . type_map [i] as usize] ; write ! (f , "{:?}, " , (std as f64 / 3600.0 , rule as f64 / 3600.0)) ? ; i += 1 ; } for & t in self . trans { dbg_timestamp (f , t as i64) ? ; let (std , rule) = self . type_offsets [self . type_map [i] as usize] ; write ! (f , "{:?}, " , (std as f64 / 3600.0 , rule as f64 / 3600.0)) ? ; i += 1 ; } for & (hi , lo) in self . trans_post32 { dbg_timestamp (f , ((hi as u32 as u64) << 32 | (lo as u32 as u64)) as i64) ? ; let (std , rule) = self . type_offsets [self . type_map [i] as usize] ; write ! (f , "{:?}, " , (std as f64 / 3600.0 , rule as f64 / 3600.0)) ? ; i += 1 ; } write ! (f , "], ") ? ; write ! (f , "}}") } }
};
}
