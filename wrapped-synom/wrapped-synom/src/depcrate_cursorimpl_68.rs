// Generated macro for impl_68 (impl)
macro_rules! Depcrate_cursorimpl_68 {
() => {
// Module: crate::cursor
// Provides: {"impl_68"}
// Dependencies: {}
impl SynomBuffer { fn inner_new (stream : TokenStream , up : * const Entry) -> SynomBuffer { let mut entries = Vec :: new () ; let mut seqs = Vec :: new () ; for tt in stream . into_iter () { match tt . kind { TokenNode :: Term (sym) => { entries . push (Entry :: Term (tt . span , sym)) ; } TokenNode :: Op (chr , ok) => { entries . push (Entry :: Op (tt . span , chr , ok)) ; } TokenNode :: Literal (lit) => { entries . push (Entry :: Literal (tt . span , lit)) ; } TokenNode :: Group (delim , seq_stream) => { seqs . push ((entries . len () , tt . span , delim , seq_stream)) ; entries . push (Entry :: End (ptr :: null ())) ; } } } entries . push (Entry :: End (up)) ; let mut entries = entries . into_boxed_slice () ; for (idx , span , delim , seq_stream) in seqs { let seq_up = & entries [idx + 1] as * const Entry ; let inner = Self :: inner_new (seq_stream , seq_up) ; entries [idx] = Entry :: Group (span , delim , inner) ; } SynomBuffer { data : entries } } # [doc = " Create a new SynomBuffer, which contains the data from the given"] # [doc = " TokenStream."] pub fn new (stream : TokenStream) -> SynomBuffer { Self :: inner_new (stream , ptr :: null ()) } # [doc = " Create a cursor referencing the first token in the input."] pub fn begin (& self) -> Cursor { unsafe { Cursor :: create (& self . data [0] , & self . data [self . data . len () - 1]) } } }
};
}
