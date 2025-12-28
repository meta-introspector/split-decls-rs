macro_rules! SetActualSpanIdError {
    () => {
        # [derive (Debug)] pub (crate) struct SetActualSpanIdError { previous_span_id : u64 , new_span_id : u64 , }
    };
}

SetActualSpanIdError!();