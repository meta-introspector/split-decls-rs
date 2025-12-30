// Generated macro for shell_execute_from_explorer (function)
macro_rules! Depcrateshell_execute_from_explorer {
() => {
// Module: crate
// Provides: {"shell_execute_from_explorer"}
// Dependencies: {}
fn shell_execute_from_explorer (file : & str , args : & str , directory : & str , operation : & str , show : SHOW_WINDOW_CMD ,) -> Result < () > { unsafe { let view : IShellView = find_desktop_folder_view () ? ; let background : IDispatch = view . GetItemObject (SVGIO_BACKGROUND) ? ; let folder : IShellFolderViewDual = background . cast () ? ; let shell : IShellDispatch2 = folder . Application () ? . cast () ? ; shell . ShellExecute (& BSTR :: from (file) , & VARIANT :: from (args) , & VARIANT :: from (directory) , & VARIANT :: from (operation) , & VARIANT :: from (show . 0) ,) } }
};
}
