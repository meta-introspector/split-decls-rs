// Generated macro for SysvarSerialize (trait)
macro_rules! DepcrateSysvarSerialize {
() => {
// Module: crate
// Provides: {"SysvarSerialize"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " A type that holds sysvar data."] pub trait SysvarSerialize : Sysvar + SysvarId + serde :: Serialize + serde :: de :: DeserializeOwned { # [doc = " The size in bytes of the sysvar as serialized account data."] fn size_of () -> usize { bincode :: serialized_size (& Self :: default ()) . unwrap () as usize } # [doc = " Deserializes the sysvar from its `AccountInfo`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If `account_info` does not have the same ID as the sysvar this function"] # [doc = " returns [`ProgramError::InvalidArgument`]."] fn from_account_info (account_info : & AccountInfo) -> Result < Self , ProgramError > { if ! Self :: check_id (account_info . unsigned_key ()) { return Err (ProgramError :: InvalidArgument) ; } bincode :: deserialize (& account_info . data . borrow ()) . map_err (| _ | ProgramError :: InvalidArgument) } # [doc = " Serializes the sysvar to `AccountInfo`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns `None` if serialization failed."] fn to_account_info (& self , account_info : & mut AccountInfo) -> Option < () > { bincode :: serialize_into (& mut account_info . data . borrow_mut () [..] , self) . ok () } }
};
}
