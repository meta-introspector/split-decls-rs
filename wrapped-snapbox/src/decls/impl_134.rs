macro_rules! deps {
    () => {
        Error!();
        DataInner!();
        IntoJson!();
        FilterSet!();
        IntoData!();
        Data!();
        DataError!();
        DataFormat!();
        ToDebug!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        # [doc = " # Constructors"] # [doc = ""] # [doc = " See also"] # [doc = " - [`str!`] for inline snapshots"] # [doc = " - [`file!`] for external snapshots"] # [doc = " - [`ToString`] for verifying a `Display` representation"] # [doc = " - [`ToDebug`] for verifying a debug representation"] # [doc = " - [`IntoJson`] for verifying the serde representation"] # [doc = " - [`IntoData`] for modifying `expected`"] impl Data { # [doc = " Mark the data as binary (no post-processing)"] pub fn binary (raw : impl Into < Vec < u8 > >) -> Self { Self :: with_inner (DataInner :: Binary (raw . into ())) } # [doc = " Mark the data as text (post-processing)"] pub fn text (raw : impl Into < String >) -> Self { Self :: with_inner (DataInner :: Text (raw . into ())) } # [cfg (feature = "json")] pub fn json (raw : impl Into < serde_json :: Value >) -> Self { Self :: with_inner (DataInner :: Json (raw . into ())) } # [cfg (feature = "json")] pub fn jsonlines (raw : impl Into < Vec < serde_json :: Value > >) -> Self { Self :: with_inner (DataInner :: JsonLines (serde_json :: Value :: Array (raw . into ()))) } fn error (raw : impl Into < crate :: assert :: Error > , intended : DataFormat) -> Self { Self :: with_inner (DataInner :: Error (DataError { error : raw . into () , intended , })) } # [doc = " Empty test data"] pub fn new () -> Self { Self :: text ("") } # [doc = " Load `expected` data from a file"] pub fn read_from (path : & std :: path :: Path , data_format : Option < DataFormat >) -> Self { match Self :: try_read_from (path , data_format) { Ok (data) => data , Err (err) => Self :: error (err , data_format . unwrap_or_else (| | DataFormat :: from (path))) . with_path (path) , } } # [doc = " Remove default [`filters`][crate::filter] from this `expected` result"] pub fn raw (mut self) -> Self { self . filters = FilterSet :: empty () . newlines () ; self } # [doc = " Treat lines and json arrays as unordered"] pub fn unordered (mut self) -> Self { self . filters = self . filters . unordered () ; self } }
    };
}

impl_134!();