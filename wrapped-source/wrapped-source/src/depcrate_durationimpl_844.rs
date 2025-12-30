// Generated macro for impl_844 (impl)
macro_rules! Depcrate_durationimpl_844 {
() => {
// Module: crate::duration
// Provides: {"impl_844"}
// Dependencies: {}
# [cfg (feature = "experimental")] impl DataProvider < DigitalDurationDataV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < DigitalDurationDataV1 > , DataError > { let (hm_hour_pad , hm_min_pad , hm_sep , ms_min_pad , ms_sec_pad , hms_hour_pad , hms_min_pad , hms_sec_pad ,) = self . load_duration_parts_internal (req) ? ; let result = DigitalDurationData { separator : Cow :: Owned (hm_sep . to_string ()) , hms_padding : HmsPadding { h : hms_hour_pad , m : hms_min_pad , s : hms_sec_pad , } , hm_padding : HmPadding { h : hm_hour_pad , m : hm_min_pad , } , ms_padding : MsPadding { m : ms_min_pad , s : ms_sec_pad , } , } ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (result) , }) } }
};
}
