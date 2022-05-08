use std::collections::BTreeMap;

use byteorder::{ByteOrder, LE};

pub enum Value {
  Null,
  Boolean(bool),
  Number(i64),
  String(String),
  Uint8Array(Vec<u8>),
  Array(Vec<Value>),
  Map(BTreeMap<String, Value>), // sort.Strings(keys)
}

pub struct Packet {
  id: u32,
  is_request: bool,
  value: Value,
}

impl Packet {
  pub fn encode(self) -> Vec<u8> {
    let mut bytes = Vec::new();
    fn visit(value: Value, bytes: &mut Vec<u8>) {
      match value {
        Value::Null => {
          bytes.push(0);
        }
        Value::Boolean(value) => {
          bytes.push(1);
          bytes.push(if value { 1 } else { 0 });
        }
        Value::Number(value) => {
          bytes.push(2);
          write_u32(bytes, value as u32);
        }
        Value::String(value) => {
          bytes.push(3);
          write_u32(bytes, value.len() as u32);
          bytes.extend(value.into_bytes());
        }
        Value::Uint8Array(value) => {
          bytes.push(4);
          write_u32(bytes, value.len() as u32);
          bytes.extend(value);
        }
        Value::Array(value) => {
          bytes.push(5);
          write_u32(bytes, value.len() as u32);
          for item in value {
            visit(item, bytes);
          }
        }
        Value::Map(map) => {
          bytes.push(6);
          write_u32(bytes, map.len() as u32);
          for (key, value) in map {
            bytes.extend(key.into_bytes());
            visit(value, bytes);
          }
        }
      }
    }

    let mut body = Vec::new();
    if self.is_request {
      write_u32(&mut body, self.id << 1);
    } else {
      write_u32(&mut body, (self.id << 1) | 1);
    }
    visit(self.value, &mut body);
    write_u32(&mut bytes, body.len() as u32);
    bytes.extend(body);
    bytes
  }
}

fn write_u32(bytes: &mut Vec<u8>, value: u32) {
  bytes.extend([0; 4]);
  let len = bytes.len();
  LE::write_u32(&mut bytes[len - 4..], value);
}
