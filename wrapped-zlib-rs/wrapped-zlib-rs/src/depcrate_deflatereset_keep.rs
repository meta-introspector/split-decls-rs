// Generated macro for reset_keep (function)
macro_rules! Depcrate_deflatereset_keep {
() => {
// Module: crate::deflate
// Provides: {"reset_keep"}
// Dependencies: {}
fn reset_keep (stream : & mut DeflateStream) -> ReturnCode { stream . total_in = 0 ; stream . total_out = 0 ; stream . msg = core :: ptr :: null_mut () ; stream . data_type = crate :: c_api :: Z_UNKNOWN ; let state = & mut stream . state ; state . bit_writer . pending . reset_keep () ; state . wrap = state . wrap . abs () ; state . status = match state . wrap { 2 => Status :: GZip , _ => Status :: Init , } ; stream . adler = match state . wrap { 2 => { state . crc_fold = Crc32Fold :: new () ; CRC32_INITIAL_VALUE as _ } _ => ADLER32_INITIAL_VALUE as _ , } ; state . last_flush = - 2 ; state . zng_tr_init () ; ReturnCode :: Ok }
};
}
