// Generated macro for Model (struct)
macro_rules! Depcrate_modelModel {
() => {
// Module: crate::model
// Provides: {"Model"}
// Dependencies: {}
# [doc = " Global model reconstruction"] # [derive (Default)] pub struct Model { # [doc = " Assignment of the global model."] # [doc = ""] # [doc = " Whenever the solver state is SAT this must be up to date."] assignment : Vec < Option < bool > > , }
};
}
