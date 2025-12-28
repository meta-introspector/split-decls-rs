macro_rules! deps {
    () => {
        ErrorSample!();
        PublicSymbol!();
        FileMetadata!();
        Declaration!();
    };
}

macro_rules! ExtractionResult {
    () => {
        deps!();
        # [derive (Debug)] pub struct ExtractionResult { pub declarations : HashMap < String , Declaration > , pub errors : Vec < ErrorSample > , pub file_metadata : FileMetadata , pub public_symbols : Vec < PublicSymbol > , }
    };
}

ExtractionResult!();