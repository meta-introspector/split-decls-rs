// Generated macro for SeqAccess (struct)
macro_rules! DepcrateSeqAccess {
() => {
// Module: crate
// Provides: {"SeqAccess"}
// Dependencies: {}
# [doc = " Seq visitor that tracks the index of its elements."] struct SeqAccess < 'a , 'b , X , F : 'b > { delegate : X , callback : & 'b mut F , path : & 'a Path < 'a > , index : usize , }
};
}
