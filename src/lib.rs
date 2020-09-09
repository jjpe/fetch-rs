#![allow(unused_unsafe)] // HACK: Rust can't handle attribute macros on exprs

#[cfg(all(target_arch = "wasm32"))]
#[macro_use]
pub mod raw;

#[cfg(all(target_arch = "wasm32"))]
#[macro_use]
pub mod log;

#[cfg(all(target_arch = "wasm32"))] use std::borrow::Cow;
#[cfg(all(target_arch = "wasm32"))] use std::collections::HashMap;
#[cfg(all(target_arch = "wasm32"))] use wasm_bindgen::prelude::*;


#[cfg(all(target_arch = "wasm32"))]
mod examples {
    use super::*;


    #[wasm_bindgen]
    #[derive(Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
    pub struct Msg {
        id: usize,
        name: String,
        stock: usize,
        description: String,
        price: f32,
        currency: String,
        created_at: String,
        image_paths: Vec<String>,
    }

    /// Suppose that sending a POST request to https://myawesomesite/article/1
    /// yields the following JSON response:
    ///
    /// {
    ///   id: 1,
    ///   name: "Test Article #00",
    ///   stock: 42,
    ///   description: "This marvellous article is made of hopes, dreams, and lots of time.",
    ///   price: 300.42,
    ///   currency: "euro",
    ///   created_at: "2020-08-17T11:40:07Z",
    ///   image_paths: [
    ///       "/img/articles/1/logo.63a7d78d.svg",
    ///       "/img/articles/1/logo.82b9c7a5.png",
    ///   ],
    /// }
    ///
    /// Note how the types of the `Msg` type's fields correspond to the
    /// values in the JSON message.
    ///
    /// Then that JSON object can be deserialized to the `Msg` type as follows:
    #[wasm_bindgen] // <- Allow calling this fn from JavaScript land for easy testing
    #[allow(non_snake_case)]
    pub async fn test__json_post_request() -> Result<JsValue, JsValue> {
        // 1. Fetch a `url` using the `POST` method and deserialize the
        //    response body to JSON, and then to an instance of type `Foo`:
        let object = Request::post("https://myawesomesite/article/1")
        //  .with_header("foo", "bar") // <- Attach a header, can be done repeatedly
            .fetch_json::<Msg>()
            .await?;
        // 2. Print a message to console.log():
        info!("[test__json_post_request] body object: {:#?}\n\n\n", object);
        // 3. Serialize the `object` to JSON so that the JS world can read it:
        JsValue::from_serde(&object).map_err(|err| JsValue::from_str(
            &format!("failed to deserialize: {}", err)
        ))
    }

    /// This example shows how to fetch the HTML resource
    /// located at http://google.com.
    #[wasm_bindgen] // <- Allow calling this fn from JavaScript land for easy testing
    #[allow(non_snake_case)]
    pub async fn test__text_get_request() -> Result<JsValue, JsValue> {
        // 1. Fetch a `url` using the `GET` method and deserialize the
        //    response body text to a `String`:
        let text = Request::get("http://google.com")
        //  .with_header("foo", "bar") // <- Attach a header, can be done repeatedly
            .fetch_text()
            .await?;
        // 2. Print a message to console.log():
        info!("[test__text_get_request] text body: {:#?}\n\n\n", text);
        // 3. Serialize the `object` to JSON so that the JS world can read it:
        JsValue::from_serde(&text).map_err(|err| JsValue::from_str(
            &format!("failed to deserialize: {}", err)
        ))
    }

}


#[cfg(all(target_arch = "wasm32"))]
pub struct Request {
    url: String,
    headers: Headers,
    options: Options,
}

#[cfg(all(target_arch = "wasm32"))]
impl Request {
    #[inline]
    pub fn get<S: Into<String>>(url: S) -> Self {
        Self::new(url).with_method(Method::Get)
    }

    #[inline]
    pub fn post<S: Into<String>>(url: S) -> Self {
        Self::new(url).with_method(Method::Post)
    }

    fn new<S: Into<String>>(url: S) -> Self {
        Self {
            url: url.into(),
            headers: Headers::new(),
            options: Options::new(),
        }
    }

    fn with_method(mut self, method: Method) -> Self {
        self.options.method(method);
        self
    }

    pub fn with_header<N, V>(mut self, name: N, value: V) -> Self
    where N: Into<Cow<'static, str>>,
          V: Into<Cow<'static, str>> {
        self.headers.add(name, value);
        self
    }

    pub async fn fetch_text(self) -> Result<String, JsValue> {
        let url = self.url.as_ref();
        let options = object! {
            "method" => self.options.method.as_str(),
            // TODO: more options
        };
        let val: JsValue = unsafe { raw::fetch_text(url, Some(options)).await };
        val.into_serde::<String>().map_err(|err| JsValue::from_str(
            &format!("failed to deserialize: {}", err)
        ))
    }

    pub async fn fetch_json<T>(self) -> Result<T, JsValue>
    where T: for<'de> serde::Deserialize<'de> {
        let url = self.url.as_ref();
        let options = object! {
            "method" => self.options.method.as_str(),
            // TODO: more options
        };
        let val: JsValue = unsafe { raw::fetch_json(url, Some(options)).await };
        val.into_serde::<T>().map_err(|err| JsValue::from_str(
            &format!("failed to deserialize: {}", err)
        ))
    }
}




#[cfg(all(target_arch = "wasm32"))]
pub struct Options {
    method: Method,
}

#[cfg(all(target_arch = "wasm32"))]
impl Options {
    pub fn new() -> Self {
        Self {
            method: Method::Get,
        }
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }
}



/// See [the MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
/// for more information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl Method {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Get     => "Get",
            Self::Head    => "Head",
            Self::Post    => "Post",
            Self::Put     => "Put",
            Self::Delete  => "Delete",
            Self::Connect => "Connect",
            Self::Options => "Options",
            Self::Trace   => "Trace",
            Self::Patch   => "Patch",
        }
    }
}



#[cfg(all(target_arch = "wasm32"))]
pub struct Headers(HashMap<Cow<'static, str>, Cow<'static, str>>);

#[cfg(all(target_arch = "wasm32"))]
impl Headers {
    pub fn new() -> Self { Self(HashMap::new()) }

    pub fn add<N, V>(&mut self, name: N, value: V) -> &mut Self
    where N: Into<Cow<'static, str>>,
          V: Into<Cow<'static, str>> {
        self.0.insert(name.into(), value.into());
        self
    }
}
