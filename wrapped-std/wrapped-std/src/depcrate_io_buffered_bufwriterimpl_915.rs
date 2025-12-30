// Generated macro for impl_915 (impl)
macro_rules! Depcrate_io_buffered_bufwriterimpl_915 {
() => {
// Module: crate::io::buffered::bufwriter
// Provides: {"impl_915"}
// Dependencies: {}
# [stable (feature = "bufwriter_into_parts" , since = "1.56.0")] impl fmt :: Debug for WriterPanicked { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WriterPanicked") . field ("buffer" , & format_args ! ("{}/{}" , self . buf . len () , self . buf . capacity ())) . finish () } }
};
}
