// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { unsafe { CoInitializeEx (None , COINIT_MULTITHREADED) . ok () ? ; let window = FindWindowA (None , s ! ("Calculator")) ? ; let automation : IUIAutomation = CoCreateInstance (& CUIAutomation , None , CLSCTX_ALL) ? ; let element : IUIAutomationElement = automation . ElementFromHandle (window) ? ; let name = element . CurrentName () ? ; println ! ("window name: {name:?}") ; let element : Result < AutomationElement > = element . cast () ; if let Ok (element) = element { println ! ("file name: {:?}" , element . ExecutableFileName () ?) ; } } Ok (()) }
};
}
