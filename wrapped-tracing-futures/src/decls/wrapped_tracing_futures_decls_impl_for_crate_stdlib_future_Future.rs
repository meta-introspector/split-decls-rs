use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(feature = "std-future", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "std-future", feature = "std"))))]
impl<T: crate::stdlib::future::Future> crate::stdlib::future::Future for WithDispatch<T> {
    type Output = T::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> crate::stdlib::task::Poll<Self::Output> {
        let this = self.project();
        let dispatch = this.dispatch;
        let future = this.inner;
        dispatcher::with_default(dispatch, || future.poll(cx))
    }
}
