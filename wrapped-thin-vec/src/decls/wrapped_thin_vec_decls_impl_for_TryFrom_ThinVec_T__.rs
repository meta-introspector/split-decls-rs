use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, const N: usize> TryFrom<ThinVec<T>> for [T; N] {
    type Error = ThinVec<T>;
    /// Gets the entire contents of the `ThinVec<T>` as an array,
    /// if its size exactly matches that of the requested array.
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    /// use std::convert::TryInto;
    ///
    /// assert_eq!(thin_vec![1, 2, 3].try_into(), Ok([1, 2, 3]));
    /// assert_eq!(<ThinVec<i32>>::new().try_into(), Ok([]));
    /// ```
    ///
    /// If the length doesn't match, the input comes back in `Err`:
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    /// use std::convert::TryInto;
    ///
    /// let r: Result<[i32; 4], _> = (0..10).collect::<ThinVec<_>>().try_into();
    /// assert_eq!(r, Err(thin_vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
    /// ```
    ///
    /// If you're fine with just getting a prefix of the `ThinVec<T>`,
    /// you can call [`.truncate(N)`](ThinVec::truncate) first.
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    /// use std::convert::TryInto;
    ///
    /// let mut v = ThinVec::from("hello world");
    /// v.sort();
    /// v.truncate(2);
    /// let [a, b]: [_; 2] = v.try_into().unwrap();
    /// assert_eq!(a, b' ');
    /// assert_eq!(b, b'd');
    /// ```
    fn try_from(mut vec: ThinVec<T>) -> Result<[T; N], ThinVec<T>> {
        if vec.len() != N {
            return Err(vec);
        }
        unsafe { vec.set_len(0) };
        let array = unsafe { ptr::read(vec.data_raw() as *const [T; N]) };
        Ok(array)
    }
}
