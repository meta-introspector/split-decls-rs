// Generated macro for impl_49 (impl)
macro_rules! Depcrate_complex_lstm_matriximpl_49 {
() => {
// Module: crate::complex::lstm::matrix
// Provides: {"impl_49"}
// Dependencies: {}
impl < const D : usize > MatrixOwned < D > { pub (super) fn as_borrowed (& self) -> MatrixBorrowed < '_ , D > { MatrixBorrowed { data : & self . data , dims : self . dims , } } pub (super) fn new_zero (dims : [usize ; D]) -> Self { let total_len = dims . iter () . product :: < usize > () ; MatrixOwned { data : vec ! [0.0 ; total_len] , dims , } } # [doc = " Returns the tightly packed submatrix at _index_, or `None` if _index_ is out of range."] # [doc = ""] # [doc = " For example, if the matrix is 5x4x3, this function returns a matrix sized 4x3. If the"] # [doc = " matrix is 4x3, then this function returns a linear matrix of length 3."] # [doc = ""] # [doc = " The type parameter `M` should be `D - 1`."] # [inline] pub (super) fn submatrix < const M : usize > (& self , index : usize) -> Option < MatrixBorrowed < '_ , M > > { assert_eq ! (M , D - 1) ; let (range , dims) = self . as_borrowed () . submatrix_range (index) ; let data = & self . data . get (range) ? ; Some (MatrixBorrowed { data , dims }) } pub (super) fn as_mut (& mut self) -> MatrixBorrowedMut < '_ , D > { MatrixBorrowedMut { data : & mut self . data , dims : self . dims , } } # [doc = " A mutable version of [`Self::submatrix`]."] # [inline] pub (super) fn submatrix_mut < const M : usize > (& mut self , index : usize ,) -> Option < MatrixBorrowedMut < '_ , M > > { assert_eq ! (M , D - 1) ; let (range , dims) = self . as_borrowed () . submatrix_range (index) ; let data = self . data . get_mut (range) ? ; Some (MatrixBorrowedMut { data , dims }) } }
};
}
