macro_rules! DecoderState {
    () => {
        enum DecoderState { ReadyToDecodeNextHeader , ReadyToDecodeNextBody , # [allow (dead_code)] Failed , }
    };
}

DecoderState!();