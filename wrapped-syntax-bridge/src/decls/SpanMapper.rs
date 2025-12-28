macro_rules! SpanMapper {
    () => {
        pub trait SpanMapper < S > { fn span_for (& self , range : TextRange) -> S ; }
    };
}

SpanMapper!();