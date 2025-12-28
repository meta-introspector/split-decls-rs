macro_rules! deps {
    () => {
        LocalDateTime!();
        HierarchicalLayer!();
    };
}

macro_rules! FormatTime {
    () => {
        deps!();
        # [doc = " A type that can measure and format the current time."] # [doc = ""] # [doc = " This trait is used by [HierarchicalLayer] to include a timestamp with each"] # [doc = " [Event] when it is logged."] # [doc = ""] # [doc = " Notable default implementations of this trait are [LocalDateTime] and `()`."] # [doc = " The former prints the current time as reported by [time's OffsetDateTime]"] # [doc = " (note that it requires a `time` feature to be enabled and may panic!"] # [doc = " make sure to check out the docs for the [LocalDateTime]),"] # [doc = " and the latter does not print the current time at all."] # [doc = ""] # [doc = " Inspired by the [FormatTime] trait from [tracing-subscriber]."] # [doc = ""] # [doc = " [HierarchicalLayer]: crate::HierarchicalLayer"] # [doc = " [Event]: tracing_core::Event"] # [doc = " [time's OffsetDateTime]: time::OffsetDateTime"] # [doc = " [FormatTime]: tracing_subscriber::fmt::time::FormatTime"] # [doc = " [tracing-subscriber]: tracing_subscriber"] pub trait FormatTime { fn format_time (& self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result ; fn style_timestamp (& self , ansi : bool , elapsed : Duration , w : & mut impl std :: fmt :: Write ,) -> std :: fmt :: Result ; }
    };
}

FormatTime!();