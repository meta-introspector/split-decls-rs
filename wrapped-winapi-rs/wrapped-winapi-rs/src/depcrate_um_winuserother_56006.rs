// Generated macro for other_56006 (other)
macro_rules! Depcrate_um_winuserother_56006 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56006"}
// Dependencies: {}
extern "system" { pub fn GetMenuInfo (hMenu : HMENU , lpcmi : LPMENUINFO ,) -> BOOL ; pub fn SetMenuInfo (hMenu : HMENU , lpcmi : LPCMENUINFO ,) -> BOOL ; pub fn EndMenu (hMenu : HMENU , uFlags : UINT , uIDNewItem : UINT_PTR , lpNewItem : LPCSTR ,) -> BOOL ; }
};
}
