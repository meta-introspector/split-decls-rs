// Generated macro for TransformResult (struct)
macro_rules! Depcrate_locale_likely_subtagsTransformResult {
() => {
// Module: crate::locale::likely_subtags
// Provides: {"TransformResult"}
// Dependencies: {}
# [derive (Default)] pub (crate) struct TransformResult { language_script : BTreeMap < (TinyAsciiStr < 3 > , TinyAsciiStr < 4 >) , Region > , language_region : BTreeMap < (TinyAsciiStr < 3 > , TinyAsciiStr < 3 >) , Script > , language : BTreeMap < TinyAsciiStr < 3 > , (Script , Region) > , script_region : BTreeMap < (TinyAsciiStr < 4 > , TinyAsciiStr < 3 >) , Language > , script : BTreeMap < TinyAsciiStr < 4 > , (Language , Region) > , region : BTreeMap < TinyAsciiStr < 3 > , (Language , Script) > , und : Option < (Language , Script , Region) > , }
};
}
