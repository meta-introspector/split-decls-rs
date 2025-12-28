macro_rules! deps {
    () => {
        SerializerError!();
    };
}

macro_rules! key_must_be_a_string {
    () => {
        deps!();
        # [inline] fn key_must_be_a_string () -> SerializerError { SerializerError ("Key must be a string" . to_string ()) }
    };
}

key_must_be_a_string!()