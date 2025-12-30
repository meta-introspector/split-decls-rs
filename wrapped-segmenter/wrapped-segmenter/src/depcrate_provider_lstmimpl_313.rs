// Generated macro for impl_313 (impl)
macro_rules! Depcrate_provider_lstmimpl_313 {
() => {
// Module: crate::provider::lstm
// Provides: {"impl_313"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de : 'data , 'data > serde :: Deserialize < 'de > for LstmDataFloat32 < 'data > { fn deserialize < S > (deserializer : S) -> Result < Self , S :: Error > where S : serde :: de :: Deserializer < 'de > , { # [derive (serde :: Deserialize)] struct Raw < 'data > { model : ModelType , # [cfg_attr (feature = "serde" , serde (borrow))] dic : ZeroMap < 'data , PotentialUtf8 , u16 > , # [cfg_attr (feature = "serde" , serde (borrow))] embedding : LstmMatrix2 < 'data > , # [cfg_attr (feature = "serde" , serde (borrow))] fw_w : LstmMatrix3 < 'data > , # [cfg_attr (feature = "serde" , serde (borrow))] fw_u : LstmMatrix3 < 'data > , # [cfg_attr (feature = "serde" , serde (borrow))] fw_b : LstmMatrix2 < 'data > , # [cfg_attr (feature = "serde" , serde (borrow))] bw_w : LstmMatrix3 < 'data > , # [cfg_attr (feature = "serde" , serde (borrow))] bw_u : LstmMatrix3 < 'data > , # [cfg_attr (feature = "serde" , serde (borrow))] bw_b : LstmMatrix2 < 'data > , # [cfg_attr (feature = "serde" , serde (borrow))] time_w : LstmMatrix3 < 'data > , # [cfg_attr (feature = "serde" , serde (borrow))] time_b : LstmMatrix1 < 'data > , } let raw = Raw :: deserialize (deserializer) ? ; use serde :: de :: Error ; Self :: try_from_parts (raw . model , raw . dic , raw . embedding , raw . fw_w , raw . fw_u , raw . fw_b , raw . bw_w , raw . bw_u , raw . bw_b , raw . time_w , raw . time_b ,) . map_err (| _ | S :: Error :: custom ("Invalid dimensions")) } }
};
}
