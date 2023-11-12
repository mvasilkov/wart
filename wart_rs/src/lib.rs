use js_sys;
use wasm_bindgen::prelude::*;
use wast::lexer::Lexer;

#[wasm_bindgen]
pub fn lexer(wat: &str, callback: &js_sys::Function) {
    let lexer = Lexer::new(&wat);
    for token_result in lexer.iter(0) {
        match token_result {
            Ok(token) => {
                let kind = js_sys::JsString::from(format!("{:?}", token.kind));
                let offset = js_sys::Number::from(token.offset as u32);
                let len = js_sys::Number::from(token.len);
                let token_text = wat
                    .get(token.offset..token.offset + token.len as usize)
                    .unwrap();
                let text = js_sys::JsString::from(token_text);
                let _ = callback.apply(
                    &JsValue::NULL,
                    &js_sys::Array::of4(&kind, &offset, &len, &text),
                );
            }
            Err(err) => {
                let err = js_sys::Error::new(&err.to_string());
                let _ = callback.call1(&JsValue::NULL, &err);
                return;
            }
        }
    }
}
