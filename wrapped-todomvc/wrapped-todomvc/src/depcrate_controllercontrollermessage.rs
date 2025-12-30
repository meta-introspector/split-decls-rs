// Generated macro for ControllerMessage (enum)
macro_rules! Depcrate_controllerControllerMessage {
() => {
// Module: crate::controller
// Provides: {"ControllerMessage"}
// Dependencies: {}
# [doc = " Messages that represent the methods to be called on the Controller"] pub enum ControllerMessage { AddItem (String) , SetPage (String) , EditItemSave (String , String) , ToggleItem (String , bool) , EditItemCancel (String) , RemoveCompleted () , RemoveItem (String) , ToggleAll (bool) , }
};
}
