// Generated macro for BottLevel (enum)
macro_rules! Depcrate_bott_periodicityBottLevel {
() => {
// Module: crate::bott_periodicity
// Provides: {"BottLevel"}
// Dependencies: {}
# [doc = " Bott periodicity: K^n(X) ≅ K^{n+8}(X) for real K-theory"] # [doc = " In our context: abstraction levels repeat with period 8"] # [derive (Debug , Clone , Copy , PartialEq)] pub enum BottLevel { # [doc = " Level 0 ≅ Level 8: Concrete objects (points)"] Zero , # [doc = " Level 1 ≅ Level 9: Linear structures (vector bundles)"] One , # [doc = " Level 2 ≅ Level 10: Bilinear structures (tensor bundles)"] Two , # [doc = " Level 3 ≅ Level 11: Trilinear structures"] Three , # [doc = " Level 4 ≅ Level 12: Quaternionic structures (halfway point)"] Four , # [doc = " Level 5 ≅ Level 13: Trilinear dual"] Five , # [doc = " Level 6 ≅ Level 14: Bilinear dual"] Six , # [doc = " Level 7 ≅ Level 15: Linear dual"] Seven , }
};
}
