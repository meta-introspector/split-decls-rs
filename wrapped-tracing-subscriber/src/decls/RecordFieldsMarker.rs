macro_rules! RecordFieldsMarker {
    () => {
        # [derive (Debug)] # [doc (hidden)] pub struct RecordFieldsMarker { _p : () , }
    };
}

RecordFieldsMarker!();