// Generated macro for GeneralName (enum)
macro_rules! Depcrate_subject_nameGeneralName {
() => {
// Module: crate::subject_name
// Provides: {"GeneralName"}
// Dependencies: {}
# [derive (Clone , Copy)] pub (crate) enum GeneralName < 'a > { DnsName (untrusted :: Input < 'a >) , DirectoryName , IpAddress (untrusted :: Input < 'a >) , UniformResourceIdentifier (untrusted :: Input < 'a >) , Unsupported (u8) , }
};
}
