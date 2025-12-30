// Generated macro for RawLstmData (struct)
macro_rules! Depcrate_segmenter_lstmRawLstmData {
() => {
// Module: crate::segmenter::lstm
// Provides: {"RawLstmData"}
// Dependencies: {}
# [derive (serde :: Deserialize , Debug)] struct RawLstmData { model : String , dic : HashMap < String , u16 > , # [serde (rename = "mat1")] embedding : RawLstmMatrix , # [serde (rename = "mat2")] fw_w : RawLstmMatrix , # [serde (rename = "mat3")] fw_u : RawLstmMatrix , # [serde (rename = "mat4")] fw_b : RawLstmMatrix , # [serde (rename = "mat5")] bw_w : RawLstmMatrix , # [serde (rename = "mat6")] bw_u : RawLstmMatrix , # [serde (rename = "mat7")] bw_b : RawLstmMatrix , # [serde (rename = "mat8")] time_w : RawLstmMatrix , # [serde (rename = "mat9")] time_b : RawLstmMatrix , }
};
}
