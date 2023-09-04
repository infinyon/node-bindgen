fn napi_call_assert(status: sys::napi_status) {
    if status != sys::napi_status_napi_ok {
        let nj_status = NapiStatus::from(status);
        panic!("error executing napi call {:#?}", nj_status);
    }
}

fn napi_call_result(status: sys::napi_status) -> Result<(), NjError> {
    if status == sys::napi_status_napi_ok {
        Ok(())
    } else {
        let nj_status = NapiStatus::from(status);
        log::error!("node-bindgen error {:#?}", nj_status);
        Err(NjError::NapiCall(nj_status))
    }
}