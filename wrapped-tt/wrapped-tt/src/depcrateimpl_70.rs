// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl < S > Literal < S > { pub fn display_no_minus (& self) -> impl fmt :: Display { struct NoMinus < 'a , S > (& 'a Literal < S >) ; impl < S > fmt :: Display for NoMinus < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let symbol = self . 0 . symbol . as_str () . strip_prefix ('-') . unwrap_or (self . 0 . symbol . as_str ()) ; match self . 0 . kind { LitKind :: Byte => write ! (f , "b'{symbol}'") , LitKind :: Char => write ! (f , "'{symbol}'") , LitKind :: Integer | LitKind :: Float | LitKind :: Err (_) => write ! (f , "{symbol}") , LitKind :: Str => write ! (f , "\"{symbol}\"") , LitKind :: ByteStr => write ! (f , "b\"{symbol}\"") , LitKind :: CStr => write ! (f , "c\"{symbol}\"") , LitKind :: StrRaw (num_of_hashes) => { let num_of_hashes = num_of_hashes as usize ; write ! (f , r#"r{0:#<num_of_hashes$}"{text}"{0:#<num_of_hashes$}"# , "" , text = symbol) } LitKind :: ByteStrRaw (num_of_hashes) => { let num_of_hashes = num_of_hashes as usize ; write ! (f , r#"br{0:#<num_of_hashes$}"{text}"{0:#<num_of_hashes$}"# , "" , text = symbol) } LitKind :: CStrRaw (num_of_hashes) => { let num_of_hashes = num_of_hashes as usize ; write ! (f , r#"cr{0:#<num_of_hashes$}"{text}"{0:#<num_of_hashes$}"# , "" , text = symbol) } } ? ; if let Some (suffix) = & self . 0 . suffix { write ! (f , "{suffix}") ? ; } Ok (()) } } NoMinus (self) } }
};
}
