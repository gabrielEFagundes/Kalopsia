use shared::{data_types::BYTE, debug};

pub trait Serializer{
    fn advance(cursor: &mut usize){
        *cursor+=1;
    }

    /// Serializes 4-byte integer
    fn serialize_i32(buf: &mut Vec<BYTE>, i: i32){
        buf.extend_from_slice(&i.to_le_bytes());
    }

    /// Serializes a variable byte length string
    fn serialize_string(buf: &mut Vec<BYTE>, s: &String){
        Self::serialize_i32(buf, s.len() as i32);
        buf.extend_from_slice(s.as_bytes());
    }

    /// Serializes 8-byte floating point
    fn serialize_f64(buf: &mut Vec<BYTE>, f: f64){
        buf.extend_from_slice(&f.to_le_bytes());
    }

    /// Deserializes 4-byte integer, while also consuming it from the buffer
    fn deserialize_i32(buf: &mut Vec<BYTE>, cursor: &mut usize) -> i32{
        let mut bytes: [u8; 4] = [0; 4];
        for i in 0..4{
            bytes[i] = buf[*cursor];
            Self::advance(cursor);
        }
        
        i32::from_le_bytes(bytes)
    }

    /// Deserializes a variable length string, while consuming both the 
    /// string's length and all the bytes it occupies
    fn deserialize_string(buf: &mut Vec<BYTE>, cursor: &mut usize) -> String{
        let str_len: i32 = Self::deserialize_i32(buf, cursor);
        let mut str_arr: Vec<u8> = Vec::new(); // the performance cost is necessary because strings have variable length.

        for _ in 0..str_len as usize{
            str_arr.push(buf[*cursor]);
            Self::advance(cursor);
        }

        String::from_utf8(str_arr).unwrap()
    }

    /// Deserializes 8-byte floating point, while also consuming it from the buffer
    fn deserialize_f64(buf: &mut Vec<BYTE>, cursor: &mut usize) -> f64{
        let mut bytes: [u8; 8] = [0; 8];
        for i in 0..8{
            bytes[i] = buf[*cursor];
            Self::advance(cursor);
        }

        f64::from_le_bytes(bytes)
    }

    fn serialize(&self, buf: &mut Vec<BYTE>);

    fn deserialize(buf: &mut Vec<BYTE>, cursor: &mut usize) -> Self;
}