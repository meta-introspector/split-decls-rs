macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
    };
}

macro_rules! FutureIncompatibleInfo {
    () => {
        deps!();
        # [doc = " Extra information for a future incompatibility lint."] # [derive (Copy , Clone , Debug)] pub struct FutureIncompatibleInfo { # [doc = " e.g., a URL for an issue/PR/RFC or error code"] pub reference : & 'static str , # [doc = " The reason for the lint used by diagnostics to provide"] # [doc = " the right help message"] pub reason : FutureIncompatibilityReason , # [doc = " Whether to explain the reason to the user."] # [doc = ""] # [doc = " Set to false for lints that already include a more detailed"] # [doc = " explanation."] pub explain_reason : bool , # [doc = " If set to `true`, this will make future incompatibility warnings show up in cargo's"] # [doc = " reports."] # [doc = ""] # [doc = " When a future incompatibility warning is first inroduced, set this to `false`"] # [doc = " (or, rather, don't override the default). This allows crate developers an opportunity"] # [doc = " to fix the warning before blasting all dependents with a warning they can't fix"] # [doc = " (dependents have to wait for a new release of the affected crate to be published)."] # [doc = ""] # [doc = " After a lint has been in this state for a while, consider setting this to true, so it"] # [doc = " warns for everyone. It is a good signal that it is ready if you can determine that all"] # [doc = " or most affected crates on crates.io have been updated."] pub report_in_deps : bool , }
    };
}

FutureIncompatibleInfo!()