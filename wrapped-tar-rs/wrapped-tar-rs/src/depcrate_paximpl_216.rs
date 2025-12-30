// Generated macro for impl_216 (impl)
macro_rules! Depcrate_paximpl_216 {
() => {
// Module: crate::pax
// Provides: {"impl_216"}
// Dependencies: {}
# [doc = " Extension trait for `Builder` to append PAX extended headers."] impl < T : Write > crate :: Builder < T > { # [doc = " Append PAX extended headers to the archive."] # [doc = ""] # [doc = " Takes in an iterator over the list of headers to add to convert it into a header set formatted."] # [doc = ""] # [doc = " Returns io::Error if an error occurs, else it returns ()"] pub fn append_pax_extensions < 'key , 'value > (& mut self , headers : impl IntoIterator < Item = (& 'key str , & 'value [u8]) > ,) -> Result < () , io :: Error > { let mut data : Vec < u8 > = Vec :: new () ; for (key , value) in headers { let mut len_len = 1 ; let mut max_len = 10 ; let rest_len = 3 + key . len () + value . len () ; while rest_len + len_len >= max_len { len_len += 1 ; max_len *= 10 ; } let len = rest_len + len_len ; write ! (& mut data , "{} {}=" , len , key) ? ; data . extend_from_slice (value) ; data . push (b'\n') ; } if data . is_empty () { return Ok (()) ; } let mut header = crate :: Header :: new_ustar () ; let data_as_bytes : & [u8] = & data ; header . set_size (data_as_bytes . len () as u64) ; header . set_entry_type (crate :: EntryType :: XHeader) ; header . set_cksum () ; self . append (& header , data_as_bytes) } }
};
}
