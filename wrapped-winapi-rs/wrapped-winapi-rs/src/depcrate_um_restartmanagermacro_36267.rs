// Generated macro for macro_36267 (macro)
macro_rules! Depcrate_um_restartmanagermacro_36267 {
() => {
// Module: crate::um::restartmanager
// Provides: {"macro_36267"}
// Dependencies: {}
STRUCT ! { struct RM_PROCESS_INFO { Process : RM_UNIQUE_PROCESS , strAppName : [WCHAR ; CCH_RM_MAX_APP_NAME + 1] , strServiceShortName : [WCHAR ; CCH_RM_MAX_SVC_NAME + 1] , ApplicationType : RM_APP_TYPE , AppStatus : ULONG , TSSessionId : DWORD , bRestartable : BOOL , } }
};
}
