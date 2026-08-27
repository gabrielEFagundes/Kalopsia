use crate::dictionary::ValueType;

impl ValueType{
    pub fn as_str(self) -> String{
        if let ValueType::Str(s) = self { s }
        else { panic!("[ERROR] expected string"); }
    }

    pub fn as_int(self) -> i32{
        if let ValueType::Int(i) = self { i }
        else { panic!("[ERROR] expected int"); }
    }

    pub fn as_double(self) -> f64{
        if let ValueType::Double(f) = self { f }
        else { panic!("[ERROR] expected double"); }
    }

    pub fn as_vec(self) -> Vec<ValueType>{
        if let ValueType::Vec(v) = self{ v }
        else { panic!("[ERROR] expected array"); }
    }
}