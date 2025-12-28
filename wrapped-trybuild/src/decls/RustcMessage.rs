macro_rules! RustcMessage {
    () => {
        # [derive (Deserialize)] struct RustcMessage { rendered : String , level : String , }
    };
}

RustcMessage!();