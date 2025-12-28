macro_rules! deps {
    () => {
        DestructorLabel!();
    };
}

macro_rules! LocalLabel {
    () => {
        deps!();
        struct LocalLabel < 'a > { span : Span , name : & 'a str , is_generated_name : bool , is_dropped_first_edition_2024 : bool , destructors : Vec < DestructorLabel < 'a > > , }
    };
}

LocalLabel!();