macro_rules! deps {
    () => {
        LookupSpan!();
        Layered!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < 'a , L , S > LookupSpan < 'a > for Layered < L , S > where S : Subscriber + LookupSpan < 'a > , { type Data = S :: Data ; fn span_data (& 'a self , id : & span :: Id) -> Option < Self :: Data > { self . inner . span_data (id) } # [cfg (all (feature = "registry" , feature = "std"))] fn register_filter (& mut self) -> FilterId { self . inner . register_filter () } }
    };
}

impl_103!()