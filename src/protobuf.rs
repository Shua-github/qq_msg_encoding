use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Map(HashMap<u32, Value>),
    Null,
}

pub struct ProtobufEncoder;

impl ProtobufEncoder {
    pub fn encode(obj: &HashMap<u32, Value>) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut keys: Vec<&u32> = obj.keys().collect();
        keys.sort_unstable();
        for &tag in keys {
            Self::encode_field(&mut buffer, tag, &obj[&tag]);
        }
        buffer
    }

    fn encode_field(buffer: &mut Vec<u8>, tag: u32, value: &Value) {
        match value {
            Value::List(items) => {
                for item in items {
                    Self::encode_single_value(buffer, tag, item);
                }
            }
            _ => Self::encode_single_value(buffer, tag, value),
        }
    }

    fn encode_single_value(buffer: &mut Vec<u8>, tag: u32, value: &Value) {
        match value {
            Value::Null => {},
            Value::Int(v) => Self::write_key_value(buffer, tag, 0, |buf| {
                if *v < 0 {
                    let zigzag = ((v << 1) ^ (v >> 63)) as u64;
                    Self::write_varint(buf, zigzag)
                } else {
                    Self::write_varint(buf, *v as u64)
                }
            }),
            Value::Bool(b) => Self::write_key_value(buffer, tag, 0, |buf| Self::write_varint(buf, if *b { 1 } else { 0 })),
            Value::Str(s) => Self::write_key_value(buffer, tag, 2, |buf| Self::write_length_delimited(buf, s.as_bytes())),
            Value::Bytes(bytes) => Self::write_key_value(buffer, tag, 2, |buf| Self::write_length_delimited(buf, bytes)),
            Value::Map(map) => {
                let nested = Self::encode(map);
                Self::write_key_value(buffer, tag, 2, |buf| Self::write_length_delimited(buf, &nested));
            }
            _ => panic!("Unsupported encoding type"),
        }
    }

    fn write_key_value<F>(buffer: &mut Vec<u8>, tag: u32, wire_type: u8, write_value: F)
    where
        F: FnOnce(&mut Vec<u8>),
    {
        let key = (tag << 3) | wire_type as u32;
        Self::write_varint(buffer, key as u64);
        write_value(buffer);
    }

    fn write_varint(buffer: &mut Vec<u8>, mut value: u64) {
        while value >= 0x80 {
            buffer.push(((value & 0x7F) as u8) | 0x80);
            value >>= 7;
        }
        buffer.push(value as u8);
    }

    fn write_length_delimited(buffer: &mut Vec<u8>, data: &[u8]) {
        Self::write_varint(buffer, data.len() as u64);
        buffer.extend_from_slice(data);
    }
}

