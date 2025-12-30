// Generated macro for HuffmanEncoder (struct)
macro_rules! Depcrate_huff0_huff0_encoderHuffmanEncoder {
() => {
// Module: crate::huff0::huff0_encoder
// Provides: {"HuffmanEncoder"}
// Dependencies: {}
pub (crate) struct HuffmanEncoder < 'output , 'table , V : AsMut < Vec < u8 > > > { table : & 'table HuffmanTable , writer : & 'output mut BitWriter < V > , }
};
}
