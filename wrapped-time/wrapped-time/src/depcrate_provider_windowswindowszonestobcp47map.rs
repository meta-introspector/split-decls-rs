// Generated macro for WindowsZonesToBcp47Map (struct)
macro_rules! Depcrate_provider_windowsWindowsZonesToBcp47Map {
() => {
// Module: crate::provider::windows
// Provides: {"WindowsZonesToBcp47Map"}
// Dependencies: {}
# [doc = " A mapping from Windows Timezone names to the corresponding BCP-47 IDs."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (PartialEq , Debug , Clone , zerofrom :: ZeroFrom , yoke :: Yokeable)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_time :: provider :: windows))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct WindowsZonesToBcp47Map < 'data > { # [doc = " A map from a `WindowsZoneIdentifier` and `WindowsRegion` to indexes of the associated BCP-47 time zone identifiers."] # [cfg_attr (feature = "serde" , serde (borrow))] pub map : ZeroTrieSimpleAscii < ZeroVec < 'data , u8 > > , # [doc = " A sorted list of BCP-47 time zone identifiers."] # [cfg_attr (feature = "serde" , serde (borrow))] pub bcp47_ids : ZeroVec < 'data , TimeZone > , }
};
}
