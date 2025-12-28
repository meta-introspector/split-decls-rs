macro_rules! deps {
    () => {
        Field!();
        Variant!();
    };
}

macro_rules! needs_deserialize_bound {
    () => {
        deps!();
        fn needs_deserialize_bound (field : & attr :: Field , variant : Option < & attr :: Variant >) -> bool { ! field . skip_deserializing () && field . deserialize_with () . is_none () && field . de_bound () . is_none () && variant . map_or (true , | variant | { ! variant . skip_deserializing () && variant . deserialize_with () . is_none () && variant . de_bound () . is_none () }) }
    };
}

needs_deserialize_bound!();