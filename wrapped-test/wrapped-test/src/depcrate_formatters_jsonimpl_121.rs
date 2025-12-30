// Generated macro for impl_121 (impl)
macro_rules! Depcrate_formatters_jsonimpl_121 {
() => {
// Module: crate::formatters::json
// Provides: {"impl_121"}
// Dependencies: {}
impl < S : AsRef < str > > std :: fmt :: Display for EscapedString < S > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { let mut start = 0 ; for (i , byte) in self . 0 . as_ref () . bytes () . enumerate () { let escaped = match byte { b'"' => "\\\"" , b'\\' => "\\\\" , b'\x00' => "\\u0000" , b'\x01' => "\\u0001" , b'\x02' => "\\u0002" , b'\x03' => "\\u0003" , b'\x04' => "\\u0004" , b'\x05' => "\\u0005" , b'\x06' => "\\u0006" , b'\x07' => "\\u0007" , b'\x08' => "\\b" , b'\t' => "\\t" , b'\n' => "\\n" , b'\x0b' => "\\u000b" , b'\x0c' => "\\f" , b'\r' => "\\r" , b'\x0e' => "\\u000e" , b'\x0f' => "\\u000f" , b'\x10' => "\\u0010" , b'\x11' => "\\u0011" , b'\x12' => "\\u0012" , b'\x13' => "\\u0013" , b'\x14' => "\\u0014" , b'\x15' => "\\u0015" , b'\x16' => "\\u0016" , b'\x17' => "\\u0017" , b'\x18' => "\\u0018" , b'\x19' => "\\u0019" , b'\x1a' => "\\u001a" , b'\x1b' => "\\u001b" , b'\x1c' => "\\u001c" , b'\x1d' => "\\u001d" , b'\x1e' => "\\u001e" , b'\x1f' => "\\u001f" , b'\x7f' => "\\u007f" , _ => { continue ; } } ; if start < i { f . write_str (& self . 0 . as_ref () [start .. i]) ? ; } f . write_str (escaped) ? ; start = i + 1 ; } if start != self . 0 . as_ref () . len () { f . write_str (& self . 0 . as_ref () [start ..]) ? ; } Ok (()) } }
};
}
