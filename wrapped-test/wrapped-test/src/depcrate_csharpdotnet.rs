// Generated macro for dotnet (function)
macro_rules! Depcrate_csharpdotnet {
() => {
// Module: crate::csharp
// Provides: {"dotnet"}
// Dependencies: {}
fn dotnet () -> Command { let dotnet_cmd = match env :: var ("DOTNET_ROOT") { Ok (val) => Path :: new (& val) . join ("dotnet") , Err (_e) => "dotnet" . into () , } ; Command :: new (dotnet_cmd) }
};
}
