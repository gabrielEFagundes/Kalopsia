use shared::data_types::BYTE;

pub trait Serializer{
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

    fn serialize(&self, buf: &mut Vec<BYTE>);

    fn deserialize(buf: &[BYTE]) -> Self;
}