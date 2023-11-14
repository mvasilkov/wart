use js_sys;
use std::fmt;
use wasm_bindgen::prelude::*;
use wast::lexer::{Lexer, TokenKind};

struct _TokenKind(TokenKind);

impl fmt::Display for _TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            TokenKind::LineComment => write!(f, "LineComment"),
            TokenKind::BlockComment => write!(f, "BlockComment"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::LParen => write!(f, "LParen"),
            TokenKind::RParen => write!(f, "RParen"),
            TokenKind::String => write!(f, "String"),
            TokenKind::Id => write!(f, "Id"),
            TokenKind::Keyword => write!(f, "Keyword"),
            TokenKind::Reserved => write!(f, "Reserved"),
            TokenKind::Integer(_) => write!(f, "Integer"),
            TokenKind::Float(_) => write!(f, "Float"),
        }
    }
}

#[wasm_bindgen]
pub fn lexer(wat: &str, callback: &js_sys::Function, pos: Option<bool>) {
    let mut lexer = Lexer::new(&wat);
    lexer.allow_confusing_unicode(true);
    for token_result in lexer.iter(0) {
        match token_result {
            Ok(token) => {
                let kind = js_sys::JsString::from(_TokenKind(token.kind).to_string());
                let token_text = wat
                    .get(token.offset..token.offset + token.len as usize)
                    .unwrap();
                let text = js_sys::JsString::from(token_text);
                if let Some(false) = pos {
                    let _ = callback.call2(&JsValue::NULL, &kind, &text);
                    continue;
                }
                let offset = js_sys::Number::from(token.offset as u32);
                let len = js_sys::Number::from(token.len);
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
