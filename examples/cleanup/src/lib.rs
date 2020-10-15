use std::ptr;

use node_bindgen::derive::node_bindgen;
use node_bindgen::core::val::JsEnv;
use node_bindgen::core::NjError;

/// initialize env hook up
#[node_bindgen]
fn init(env: JsEnv) -> Result<(), NjError> {
    unsafe { env.add_env_clean_up_hook(Some(my_cleanup), ptr::null_mut())?};
    println!("init");
    Ok(())
}

unsafe extern "C" fn my_cleanup(_arg: *mut ::std::os::raw::c_void) {
    println!("I'm called from node to do cleanup");
}
