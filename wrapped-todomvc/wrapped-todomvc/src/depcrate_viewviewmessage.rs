// Generated macro for ViewMessage (enum)
macro_rules! Depcrate_viewViewMessage {
() => {
// Module: crate::view
// Provides: {"ViewMessage"}
// Dependencies: {}
# [doc = " Messages that represent the methods to be called on the View"] pub enum ViewMessage { UpdateFilterButtons (String) , ClearNewTodo () , ShowItems (ItemList) , SetItemsLeft (usize) , SetClearCompletedButtonVisibility (bool) , SetCompleteAllCheckbox (bool) , SetMainVisibility (bool) , RemoveItem (String) , EditItemDone (String , String) , SetItemComplete (String , bool) , }
};
}
