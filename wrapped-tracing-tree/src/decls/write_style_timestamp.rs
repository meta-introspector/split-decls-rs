macro_rules! write_style_timestamp {
    () => {
        fn write_style_timestamp (ansi : bool , timestamp : String , unit : & str , w : & mut impl Write ,) -> std :: fmt :: Result { write ! (w , "{timestamp}{unit}" , timestamp = styled (ansi , Style :: new () . dimmed () , timestamp) , unit = styled (ansi , Style :: new () . dimmed () , unit) ,) }
    };
}

write_style_timestamp!()