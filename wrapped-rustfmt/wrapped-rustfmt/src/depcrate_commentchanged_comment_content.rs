// Generated macro for changed_comment_content (function)
macro_rules! Depcrate_commentchanged_comment_content {
() => {
// Module: crate::comment
// Provides: {"changed_comment_content"}
// Dependencies: {}
# [doc = " Returns `true` if the two strings of code have the same payload of comments."] # [doc = " The payload of comments is everything in the string except:"] # [doc = " - actual code (not comments),"] # [doc = " - comment start/end marks,"] # [doc = " - whitespace,"] # [doc = " - '*' at the beginning of lines in block comments."] fn changed_comment_content (orig : & str , new : & str) -> bool { let code_comment_content = | code | { let slices = UngroupedCommentCodeSlices :: new (code) ; slices . filter (| (kind , _ , _) | * kind == CodeCharKind :: Comment) . flat_map (| (_ , _ , s) | CommentReducer :: new (s)) } ; let res = code_comment_content (orig) . ne (code_comment_content (new)) ; debug ! ("comment::changed_comment_content: {}\norig: '{}'\nnew: '{}'\nraw_old: {}\nraw_new: {}" , res , orig , new , code_comment_content (orig) . collect ::< String > () , code_comment_content (new) . collect ::< String > ()) ; res }
};
}
