# Hash

Hash conversions translate Rust `HashMap<String, T>` objects into simple Javascript objects, and vice-versa. `T` must also be convertible to and from JS (`T: TryIntoJs + JsValue<'a>`), and all properties of a JS object must be of the same type.

Note these conversion do not support Javascript `Map()` objects at this time, but do support any type (e.g. `array`) which is coercible to a Javascript object.