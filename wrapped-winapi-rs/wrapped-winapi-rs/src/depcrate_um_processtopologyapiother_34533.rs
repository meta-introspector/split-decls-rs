// Generated macro for other_34533 (other)
macro_rules! Depcrate_um_processtopologyapiother_34533 {
() => {
// Module: crate::um::processtopologyapi
// Provides: {"other_34533"}
// Dependencies: {}
extern "system" { pub fn GetProcessGroupAffinity (hProcess : HANDLE , GroupCount : PUSHORT , GroupArray : PUSHORT ,) -> BOOL ; pub fn GetThreadGroupAffinity (hThread : HANDLE , GroupAffinity : PGROUP_AFFINITY ,) -> BOOL ; pub fn SetThreadGroupAffinity (hThread : HANDLE , GroupAffinity : * const GROUP_AFFINITY , PreviousGroupAffinity : PGROUP_AFFINITY ,) -> BOOL ; }
};
}
