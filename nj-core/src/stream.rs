use std::ptr;
use std::fmt::Debug;

use log::debug;

use futures::Stream;
use futures::stream::StreamExt;
use pin_utils::unsafe_pinned;
use pin_utils::unsafe_unpinned;

use flv_future_core::spawn;


use crate::sys::napi_value;
use crate::val::JsEnv;
use crate::NjError;
use crate::TryIntoJs;


pub trait NjStream: Stream {

    fn js_then<F>(self, fut: F) -> JsThen<Self,F>
        where F: FnMut(Self::Item),
            Self: Sized
    {

        JsThen::new(self,fut)
    }
}

impl<T: ?Sized> NjStream for T where T: Stream {}


pub struct JsThen<St, F> {
    stream: St,
    f: F,
}

impl<St: Unpin, F> Unpin for JsThen<St, F> {}


impl<St, F> JsThen<St, F>
    where St: Stream,
          F: FnMut(St::Item)
{
    unsafe_pinned!(stream: St);
    unsafe_unpinned!(f: F);

    pub fn new(stream: St, f: F) -> JsThen<St, F> {
        Self { stream, f }
    }
}


impl<St, F> TryIntoJs for JsThen<St, F>
    where St: Stream + Send + 'static,
          F: FnMut(St::Item) + Send + 'static,
          St::Item: Debug
{


    fn try_to_js(self, _js_env: &JsEnv) -> Result<napi_value,NjError> {

        let mut stream = Box::pin(self.stream);
        let mut cb = self.f;

        spawn(async move {
            while let Some(item) = stream.next().await {
                debug!("got item: {:#?}, invoking Js callback",item);
                cb(item);
            }
           
        });

        Ok(ptr::null_mut())
        
    }
}


