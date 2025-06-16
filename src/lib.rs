mod core;
#[cfg(test)]
mod test;
#[cfg(test)]
mod rng;
mod protobuf;
use core::{MessageElement, KeyboardData};
use core::create_packet_hex;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use serde::Deserialize;

// 为结构体添加 serde 反序列化
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "message_type")]
enum InputMessage {
    User {
        peer_id: u64,
        message: Vec<RawMessageElement>,
        seq: u64,
        random_number: u32,
    },
    Group {
        peer_id: u64,
        message: Vec<RawMessageElement>,
        seq: u64,
        random_number: u32,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum RawMessageElement {
    #[serde(rename = "text")]
    Text { data: TextData },
    #[serde(rename = "keyboard")]
    Keyboard { data: KeyboardData },
}

#[derive(Deserialize)]
struct TextData {
    text: String,
}

// 将 RawMessageElement 转为 MessageElement
fn parse_message_elements(raw: Vec<RawMessageElement>) -> Vec<MessageElement> {
    raw.into_iter().map(|elem| match elem {
        RawMessageElement::Text { data } => MessageElement::Text(data.text),
        RawMessageElement::Keyboard { data } => MessageElement::Keyboard(data),
    }).collect()
}

#[unsafe(no_mangle)]
pub extern "C" fn encode_from_json(json_input: *const c_char) -> *const c_char {
    // 安全读取输入
    let c_str = unsafe {
        assert!(!json_input.is_null());
        CStr::from_ptr(json_input)
    };

    let input_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null(),
    };

    // 解析 JSON
    let parsed: InputMessage = match serde_json::from_str(input_str) {
        Ok(p) => p,
        Err(_) => return std::ptr::null(),
    };

    let (elements, peer_type, peer_id, seq, random_number) = match parsed {
        InputMessage::User {
            peer_id,
            message,
            seq,
            random_number,
        } => (parse_message_elements(message), "user".to_string(), peer_id, seq, random_number),
        InputMessage::Group {
            peer_id,
            message,
            seq,
            random_number,
        } => (parse_message_elements(message), "group".to_string(), peer_id, seq, random_number),
    };

    let hex = create_packet_hex(
        elements,
        peer_type,
        peer_id,
        seq,
        random_number,
    );


    // 返回 C 字符串
    CString::new(hex).unwrap().into_raw()
}

/// 分配指定大小的内存，返回指针
#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf); // 防止 Rust 释放它
    return ptr;
}

/// 释放指针
#[unsafe(no_mangle)]
pub extern "C" fn free_ptr(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // 让 Rust 拿回这个 CString 的所有权并自动释放
        let _ = CString::from_raw(ptr);
    }
}