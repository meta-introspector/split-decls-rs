// Generated macro for BiesIterator (struct)
macro_rules! Depcrate_complex_lstmBiesIterator {
() => {
// Module: crate::complex::lstm
// Provides: {"BiesIterator"}
// Dependencies: {}
struct BiesIterator < 'l , 'data > { segmenter : & 'l LstmSegmenter < 'data > , input_seq : core :: iter :: Enumerate < alloc :: vec :: IntoIter < u16 > > , h_bw : MatrixOwned < 2 > , curr_fw : MatrixOwned < 1 > , c_fw : MatrixOwned < 1 > , }
};
}
