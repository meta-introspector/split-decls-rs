// Generated macro for impl_47 (impl)
macro_rules! Depcrate_layerimpl_47 {
() => {
// Module: crate::layer
// Provides: {"impl_47"}
// Dependencies: {}
impl < S , F > ErrorLayer < S , F > where F : for < 'writer > FormatFields < 'writer > + 'static , S : Subscriber + for < 'span > LookupSpan < 'span > , { # [doc = " Returns a new `ErrorLayer` with the provided [field formatter]."] # [doc = ""] # [doc = " [field formatter]: tracing_subscriber::fmt::FormatFields"] pub fn new (format : F) -> Self { Self { format , get_context : WithContext (Self :: get_context) , _subscriber : PhantomData , } } fn get_context (dispatch : & Dispatch , id : & span :: Id , f : & mut dyn FnMut (& 'static Metadata < 'static > , & str) -> bool ,) { let subscriber = dispatch . downcast_ref :: < S > () . expect ("subscriber should downcast to expected type; this is a bug!") ; let span = subscriber . span (id) . expect ("registry should have a span for the current ID") ; for span in span . scope () { let cont = if let Some (fields) = span . extensions () . get :: < FormattedFields < F > > () { f (span . metadata () , fields . fields . as_str ()) } else { f (span . metadata () , "") } ; if ! cont { break ; } } } }
};
}
