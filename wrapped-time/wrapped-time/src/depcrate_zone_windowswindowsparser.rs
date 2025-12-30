// Generated macro for WindowsParser (struct)
macro_rules! Depcrate_zone_windowsWindowsParser {
() => {
// Module: crate::zone::windows
// Provides: {"WindowsParser"}
// Dependencies: {}
# [doc = " A mapper between Windows time zone identifier and a BCP-47 ID."] # [doc = ""] # [doc = " This mapper currently only supports mapping from windows time zone identifiers"] # [doc = " to BCP-47 identifiers."] # [doc = ""] # [doc = " A windows time zone may vary depending on an associated territory/region. This is represented"] # [doc = " by the internal data mapping by delimiting the windows time zone and territory/region"] # [doc = " code with a \"/\"."] # [doc = ""] # [doc = " For instance, Central Standard Time can vary depending on the provided regions listed below:"] # [doc = ""] # [doc = " - Central Standard Time/001"] # [doc = " - Central Standard Time/US"] # [doc = " - Central Standard Time/CA"] # [doc = " - Central Standard Time/MX"] # [doc = " - Central Standard Time/ZZ"] # [doc = ""] # [doc = " As such, a [`Region`] may be provided to further specify a desired territory/region when"] # [doc = " querying a BCP-47 identifier. If no region is provided or the specificity is not required,"] # [doc = " then the territory will default to the M.49 World Code, `001`."] # [derive (Debug)] pub struct WindowsParser { data : DataPayload < TimezoneIdentifiersWindowsV1 > , }
};
}
