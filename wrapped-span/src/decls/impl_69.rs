macro_rules! deps {
    () => {
        SpanData!();
        SpanMap!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        # [cfg (not (no_salsa_async_drops))] impl < S > Drop for SpanMap < S > { fn drop (& mut self) { struct SendPtr (* mut [()]) ; unsafe impl Send for SendPtr { } static SPAN_MAP_DROP_THREAD : std :: sync :: OnceLock < std :: sync :: mpsc :: Sender < (SendPtr , fn (SendPtr)) > , > = std :: sync :: OnceLock :: new () ; SPAN_MAP_DROP_THREAD . get_or_init (| | { let (sender , receiver) = std :: sync :: mpsc :: channel :: < (SendPtr , fn (SendPtr)) > () ; std :: thread :: Builder :: new () . name ("SpanMapDropper" . to_owned ()) . spawn (move | | receiver . iter () . for_each (| (b , drop) | drop (b))) . unwrap () ; sender }) . send ((unsafe { SendPtr (std :: mem :: transmute :: < * mut [(TextSize , SpanData < S >)] , * mut [()] > (Box :: < [(TextSize , SpanData < S >)] > :: into_raw (std :: mem :: take (& mut self . spans) . into_boxed_slice () ,) ,)) } , | b : SendPtr | { _ = unsafe { Box :: from_raw (std :: mem :: transmute :: < * mut [()] , * mut [(TextSize , SpanData < S >)] , > (b . 0)) } } ,)) . unwrap () ; } }
    };
}

impl_69!()