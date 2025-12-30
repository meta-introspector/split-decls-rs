// Generated macro for impl_1154 (impl)
macro_rules! Depcrate_segmenter_lstmimpl_1154 {
() => {
// Module: crate::segmenter::lstm
// Provides: {"impl_1154"}
// Dependencies: {}
impl DataProvider < SegmenterLstmAutoV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < SegmenterLstmAutoV1 > , DataError > { self . check_req :: < SegmenterLstmAutoV1 > (req) ? ; let lstm_data = self . segmenter_lstm () ? . read_and_parse_json :: < RawLstmData > (& format ! ("{}/weights.json" , req . id . marker_attributes as & str)) . map_err (| _ | DataErrorKind :: IdentifierNotFound . into_error ()) ? ; let data = lstm_data . try_convert () ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data) , }) } }
};
}
