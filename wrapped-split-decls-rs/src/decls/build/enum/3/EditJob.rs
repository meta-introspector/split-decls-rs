use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Deserialize)] # [serde (tag = "type")] pub enum EditJob { AddUse (AddUseDetails) , RemoveFunction (RemoveFunctionDetails) , ReplaceExpression (ReplaceExpressionDetails) , AddFunction (AddFunctionDetails) , AddItem (AddItemDetails) , ReplaceFileContent (ReplaceFileContentDetails) , ReplaceFileContentFromFile (ReplaceFileContentFromFileDetails) , RunSearch (RunSearchDetails) , RemoveCargoDependency (RemoveCargoDependencyDetails) , RemoveUse (RemoveUseDetails) , }
}