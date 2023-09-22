use std::ptr;
use std::convert::TryFrom;

use log::trace;

use crate::TryIntoJs;
use crate::JSValue;
use crate::sys::napi_value;
use crate::val::JsEnv;
use crate::NjError;

pub use num_bigint::*;

impl<'a> JSValue<'a> for BigInt {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn convert_to_rust(env: &'a JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        trace!("Converting JS BigInt to Rust!");

        env.assert_type(js_value, crate::sys::napi_valuetype_napi_bigint)?;
        let mut word_count = 0_usize;

        // https://nodejs.org/api/n-api.html#n_api_napi_get_value_bigint_words
        // Frist call is to figure out how long of a vec to make.
        crate::napi_call_result!(crate::sys::napi_get_value_bigint_words(
            env.inner(),
            js_value,
            ptr::null_mut(),
            &mut word_count,
            ptr::null_mut(),
        ))?;

        // Now we actually get the sign and the vector.
        let mut napi_buffer: Vec<u64> = vec![0; usize::try_from(word_count).unwrap()];
        let mut sign = 0;

        crate::napi_call_result!(crate::sys::napi_get_value_bigint_words(
            env.inner(),
            js_value,
            &mut sign,
            &mut word_count,
            napi_buffer.as_mut_ptr(),
        ))?;

        // BigInt is initialized via a little endian &[u8] so we need to build the u8s from the
        // u64s
        let mut bytes: Vec<u8> = Vec::new();
        for i in &napi_buffer {
            bytes.extend_from_slice(&i.to_le_bytes());
        }

        // The N-API documentation on the signs is lacking.
        let sign = match sign {
            0 => Sign::Plus,
            1 => Sign::Minus,
            _ => unreachable!(),
        };
        let res = BigInt::from_bytes_le(sign, &bytes);
        trace!(
            "Converted JS BigInt to Rust! words: {:#X?}, bytes: {:#?}, len: {:?}, bigint: {:#?}",
            napi_buffer,
            bytes,
            bytes.len(),
            res
        );
        Ok(res)
    }
}

impl TryIntoJs for BigInt {
    fn try_to_js(self, env: &JsEnv) -> Result<napi_value, NjError> {
        let (sign, bytes) = self.to_bytes_le();
        let mut words: Vec<u64> = Vec::new();
        use std::cmp::min;

        // bytes can be non-multiples of 8.
        for i in 0..(bytes.len() / 8 + 1) {
            let mut slice: [u8; 8] = [0; 8];

            // https://stackoverflow.com/a/29784723 seems to be the least bad way to convert a Vec
            // slice into an array :/
            for (place, element) in slice
                .iter_mut()
                .zip(bytes[i * 8..min((i + 1) * 8, bytes.len())].iter())
            {
                *place = *element;
            }
            words.push(u64::from_le_bytes(slice));
        }
        let sign = match sign {
            Sign::Minus => 1,
            Sign::Plus | Sign::NoSign => 0,
        };
        let word_count = words.len();

        trace!(
            "Converted Rust BigInt to JS Bigint: {:#?}!, bytes: {:#?}, len: {:?}, words: {:#?}, word_count {:#?}, sign: {:#?}",
            self,
            bytes,
            bytes.len(),
            words,
            word_count,
            sign,
        );

        let mut napi_buffer = ptr::null_mut();

        // https://nodejs.org/api/n-api.html#n_api_napi_create_bigint_words
        crate::napi_call_result!(crate::sys::napi_create_bigint_words(
            env.inner(),
            sign,
            word_count,
            words.as_ptr(),
            &mut napi_buffer
        ))?;
        Ok(napi_buffer)
    }
}
