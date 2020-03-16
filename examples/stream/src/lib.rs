
#![feature(prelude_import)]

use std::time::Duration;

use futures::Stream;
use futures::stream;
use futures::stream::StreamExt;


use flv_future_core::sleep;
use node_bindgen::core::NjError;
use node_bindgen::derive::node_bindgen;
use node_bindgen::core::stream::NjStream;
use node_bindgen::core::stream::JsThen ;

struct StreamFactory {}



#[node_bindgen]
impl StreamFactory {

    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self{}
    }

    /// send back to nodejs using data as event
    #[node_bindgen(mt)]
    fn stream<F: Fn(String, i32)>(&self,count: i32, cb: F) -> Result<JsThen<impl Stream<Item=i32>,impl FnMut(i32)>,NjError>
    {
        // only allow count to be less than 10
        if count > 10 {
            return Err(NjError::Other(format!("count: {} should be less than or equal to 10",count)))
        }
        let stream = test_stream(count);
        let event = "data".to_owned();
        // println!("got stream with len: {}",count);
        Ok(stream.js_then( move |msg| {
            println!("sending to js callback");
            cb(event.clone(),msg); 
        }))
    }

}


// stream that generates count from 0..count with 100 milliseconds duration
fn test_stream(count: i32) -> impl Stream<Item=i32>  {

    stream::iter(0..count).then( |index| async move {
        sleep(Duration::from_millis(100)).await;
        index
    })
}