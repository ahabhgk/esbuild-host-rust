use std::collections::BTreeMap;

use byteorder::{ByteOrder, LE};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
  Null,
  Boolean(bool),
  Number(i32),
  String(String),
  Uint8Array(Vec<u8>),
  Array(Vec<Value>),
  Map(BTreeMap<String, Value>), // sort.Strings(keys)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
            write_u32(bytes, key.len() as u32);
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

  pub fn decode(bytes: Vec<u8>) -> Self {
    fn visit(bytes: &[u8]) -> (Value, &[u8]) {
      let kind = bytes[0];
      let bytes = &bytes[1..];
      match kind {
        0 => (Value::Null, bytes),
        1 => {
          let value = bytes[0];
          let next = &bytes[1..];
          (Value::Boolean(value != 0), next)
        }
        2 => {
          let (value, next) = read_u32(bytes);
          (Value::Number(value as i32), next)
        }
        3 => {
          let (value, next) = read_length_prefixed(bytes);
          (
            Value::String(String::from_utf8(value.to_vec()).unwrap()),
            next,
          )
        }
        4 => {
          let (value, next) = read_length_prefixed(bytes);
          (Value::Uint8Array(value.to_vec()), next)
        }
        5 => {
          let (len, mut next) = read_u32(bytes);
          let mut value = Vec::with_capacity(len as usize);
          for _ in 0..len {
            let (v, n) = visit(next);
            next = n;
            value.push(v);
          }
          (Value::Array(value), next)
        }
        6 => {
          let (len, mut next) = read_u32(bytes);
          let mut value = BTreeMap::new();
          for _ in 0..len {
            let (key, n) = read_length_prefixed(next);
            next = n;
            let k = String::from_utf8(key.to_vec()).unwrap();
            let (v, n) = visit(next);
            next = n;
            value.insert(k, v);
          }
          (Value::Map(value), next)
        }
        _ => panic!("Invalid packet"),
      }
    }
    let (mut id, next) = read_u32(&bytes);
    let is_request = (id & 1) == 0;
    id >>= 1;
    let (value, next) = visit(next);
    if !next.is_empty() {
      panic!("Invalid packet");
    }
    Self {
      id,
      is_request,
      value,
    }
  }
}

fn write_u32(bytes: &mut Vec<u8>, value: u32) {
  bytes.extend([0; 4]);
  let len = bytes.len();
  LE::write_u32(&mut bytes[len - 4..], value);
}

fn read_u32(bytes: &[u8]) -> (u32, &[u8]) {
  (LE::read_u32(bytes), &bytes[4..])
}

fn read_length_prefixed(bytes: &[u8]) -> (&[u8], &[u8]) {
  let (len, next) = read_u32(bytes);
  (&next[..len as usize], &next[len as usize..])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_encode_decode() {
    let value = Value::Map(BTreeMap::from_iter(vec![
      ("short".to_string(), Value::Null),
      ("longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong".to_string(), Value::Null),
      ("a".to_string(), Value::Number(0)),
      ("b".to_string(), Value::Number(-1)),
      ("c".to_string(), Value::Number(1)),
      ("d".to_string(), Value::Boolean(false)),
      ("e".to_string(), Value::Boolean(true)),
      ("f".to_string(), Value::String("?".to_string())),
      ("g".to_string(), Value::String("longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong".to_string())),
      ("h".to_string(), Value::Uint8Array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
      ("i".to_string(), Value::Array(vec![Value::Null, Value::Number(0), Value::Number(-1), Value::Boolean(false), Value::Boolean(true), Value::String("?".to_string()), Value::String("zzz".to_string()), Value::Uint8Array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])])),
      ("j".to_string(), Value::Map(BTreeMap::from_iter(vec![
        ("k".to_string(), Value::Null),
        ("l".to_string(), Value::Number(0)),
        ("m".to_string(), Value::Number(-1)),
        ("n".to_string(), Value::Number(1)),
        ("o".to_string(), Value::Boolean(false)),
        ("p".to_string(), Value::Boolean(true)),
        ("q".to_string(), Value::String("?".to_string())),
        ("r".to_string(), Value::String("longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong".to_string())),
        ("s".to_string(), Value::Uint8Array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
        ("t".to_string(), Value::Array(vec![Value::Null, Value::Number(0), Value::Number(-1), Value::Boolean(false), Value::Boolean(true), Value::String("?".to_string()), Value::String("zzz".to_string()), Value::Uint8Array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])])),
      ]))),
    ]));
    let p1 = Packet {
      id: 1,
      is_request: true,
      value,
    };
    let buf = p1.clone().encode();
    let (buf, _) = read_length_prefixed(&buf);
    let p2 = Packet::decode(buf.to_vec());
    assert_eq!(p1, p2);
  }
}
