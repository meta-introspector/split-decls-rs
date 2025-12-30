// Generated macro for LocaleResource (struct)
macro_rules! Depcrate_cldr_serde_locale_resourceLocaleResource {
() => {
// Module: crate::cldr_serde::locale_resource
// Provides: {"LocaleResource"}
// Dependencies: {}
# [doc = " Deserializer for the top layers of most CLDR JSON locale resources."] # [doc = ""] # [doc = " Most locale-specific resources have the following structure:"] # [doc = ""] # [doc = " ```json"] # [doc = " {"] # [doc = "   \"main\": {"] # [doc = "     \"en-US\": {"] # [doc = "       /* resource-specific fields */"] # [doc = "     }"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This deserializer is generic over `T`, which is the type of the resource-specific fields,"] # [doc = " and will in effect \"strip\" the top two layers from the JSON when parsing."] # [derive (Debug , Deserialize)] pub (crate) struct LocaleResource < T > { pub (crate) main : SingleLocaleMap < T > , }
};
}
