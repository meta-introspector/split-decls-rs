macro_rules! deps {
    () => {
        Field!();
        Variant!();
    };
}

macro_rules! needs_serialize_bound {
    () => {
        deps!();
        fn needs_serialize_bound (field : & attr :: Field , variant : Option < & attr :: Variant >) -> bool { ! field . skip_serializing () && field . serialize_with () . is_none () && field . ser_bound () . is_none () && variant . map_or (true , | variant | { ! variant . skip_serializing () && variant . serialize_with () . is_none () && variant . ser_bound () . is_none () }) }
    };
}

needs_serialize_bound!();