use std::collections::HashSet;

pub type BYTE = u8;

#[derive(Debug, Clone)]
pub enum ValueType {
    Str(String),
    Int(i32),
    Double(f64),
    Vec(HashSet<ValueType>),
    DVec(Vec<ValueType>),

    Default,
}

impl ValueType {
    pub fn as_str(self) -> String {
        if let ValueType::Str(s) = self {
            s
        } else {
            panic!("[ERROR] expected string");
        }
    }

    pub fn as_ref_str(&self) -> &String{
        if let ValueType::Str(s) = self{
            &s
        } else {
            panic!("[ERROR] expected string reference");
        }
    }

    pub fn as_int(self) -> i32 {
        if let ValueType::Int(i) = self {
            i
        } else {
            panic!("[ERROR] expected int");
        }
    }

    pub fn as_ref_int(&self) -> &i32{
        if let ValueType::Int(i) = self{
            &i
        } else {
            panic!("[ERROR] expected int reference");
        }
    }

    pub fn as_double(self) -> f64 {
        if let ValueType::Double(f) = self {
            f
        } else {
            panic!("[ERROR] expected double");
        }
    }

    pub fn as_ref_double(&self) -> &f64{
        if let ValueType::Double(f) = self{
            &f
        } else {
            panic!("[ERROR] expected double reference");
        }
    }

    pub fn as_hashset(self) -> HashSet<ValueType> {
        if let ValueType::Vec(v) = self {
            v
        } else {
            panic!("[ERROR] expected array");
        }
    }
}
