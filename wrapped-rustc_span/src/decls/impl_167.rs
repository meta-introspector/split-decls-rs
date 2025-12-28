macro_rules! deps {
    () => {
        SpanInterner!();
        SpanData!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl SpanInterner { fn intern (& mut self , span_data : & SpanData) -> u32 { let (index , _) = self . spans . insert_full (* span_data) ; index as u32 } }
    };
}

impl_167!()