macro_rules! Symbol {
    () => {
        # [doc = " An interned UTF-8 string."] # [doc = ""] # [doc = " Internally, a `Symbol` is implemented as an index, and all operations"] # [doc = " (including hashing, equality, and ordering) operate on that index. The use"] # [doc = " of `rustc_index::newtype_index!` means that `Option<Symbol>` only takes up 4 bytes,"] # [doc = " because `rustc_index::newtype_index!` reserves the last 256 values for tagging purposes."] # [doc = ""] # [doc = " Note that `Symbol` cannot directly be a `rustc_index::newtype_index!` because it"] # [doc = " implements `fmt::Debug`, `Encodable`, and `Decodable` in special ways."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Symbol (SymbolIndex) ;
    };
}

Symbol!();