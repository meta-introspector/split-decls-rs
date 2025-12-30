// Generated macro for impl_34 (impl)
macro_rules! Depcrate_commandsimpl_34 {
() => {
// Module: crate::commands
// Provides: {"impl_34"}
// Dependencies: {}
impl Run for Command { fn run (self) -> crate :: Result < () > { match self { Command :: CI => ci () , Command :: Build => build () , Command :: Check (command) => command . run () , Command :: CheckBackend (command) => command . run () , Command :: Readme (command) => command . run () , Command :: Coverage (command) => command . run () , Command :: Lint => lint () , Command :: Clippy (command) => command . run () , Command :: Docs (command) => command . run () , Command :: Format (command) => command . run () , Command :: Typos (command) => command . run () , Command :: LintMarkdown => lint_markdown () , Command :: Test => test () , Command :: TestBackend (command) => command . run () , Command :: TestDocs => test_docs :: test_docs () , Command :: TestLibs => test_libs () , Command :: Hack => hack () , } } }
};
}
