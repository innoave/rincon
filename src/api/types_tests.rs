
use super::types::*;

#[test]
fn convert_string_value_to_string() {
    let value = Value::String("wnaow kklsnd".to_owned());
    let string = value.to_string();
    assert_eq!(String::from("wnaow kklsnd"), string);
}

#[test]
fn convert_bool_true_value_to_string() {
    let value = Value::Bool(true);
    let string = value.to_string();
    assert_eq!(String::from("true"), string);
}

#[test]
fn convert_bool_false_value_to_string() {
    let value = Value::Bool(false);
    let string = value.to_string();
    assert_eq!(String::from("false"), string);
}

#[test]
fn convert_f32_value_to_string() {
    let value = Value::F32(-241920.34f32);
    let string = value.to_string();
    assert_eq!(String::from("-241920.34"), string);
}

#[test]
fn convert_f64_value_to_string() {
    let value = Value::F64(0.0002982838917110001f64);
    let string = value.to_string();
    assert_eq!(String::from("0.0002982838917110001"), string);
}

#[test]
fn convert_i8_value_to_string() {
    let value = Value::I8(-42i8);
    let string = value.to_string();
    assert_eq!(String::from("-42"), string);
}

#[test]
fn convert_i16_value_to_string() {
    let value = Value::I16(-42i16);
    let string = value.to_string();
    assert_eq!(String::from("-42"), string);
}

#[test]
fn convert_i32_value_to_string() {
    let value = Value::I32(-42i32);
    let string = value.to_string();
    assert_eq!(String::from("-42"), string);
}

#[test]
fn convert_i64_value_to_string() {
    let value = Value::I64(-42i64);
    let string = value.to_string();
    assert_eq!(String::from("-42"), string);
}

#[test]
fn convert_isize_value_to_string() {
    let value = Value::ISize(-42isize);
    let string = value.to_string();
    assert_eq!(String::from("-42"), string);
}

#[test]
fn convert_u8_value_to_string() {
    let value = Value::U8(42u8);
    let string = value.to_string();
    assert_eq!(String::from("42"), string);
}

#[test]
fn convert_u16_value_to_string() {
    let value = Value::U16(42u16);
    let string = value.to_string();
    assert_eq!(String::from("42"), string);
}

#[test]
fn convert_u32_value_to_string() {
    let value = Value::U32(42u32);
    let string = value.to_string();
    assert_eq!(String::from("42"), string);
}

#[test]
fn convert_u64_value_to_string() {
    let value = Value::U64(42u64);
    let string = value.to_string();
    assert_eq!(String::from("42"), string);
}

#[test]
fn convert_usize_value_to_string() {
    let value = Value::USize(42usize);
    let string = value.to_string();
    assert_eq!(String::from("42"), string);
}

#[ignore] //TODO make test pass
#[test]
fn convert_string_vec_value_to_string() {
    let value = Value::VecString(vec!["a".to_owned(), "wooxoi xcvakljs".to_owned(), "eirwo".to_owned()]);
    let string = value.to_string();
    assert_eq!(String::from(r#"["a","wooxoi xcvakljs","eirwo"]"#), string)
}

#[test]
fn convert_bool_vec_value_to_string() {
    let value = Value::VecBool(vec![true, false]);
    let string = value.to_string();
    assert_eq!(String::from(r#"[true,false]"#), string)
}

//noinspection RsApproxConstant
#[test]
fn convert_f32_vec_value_to_string() {
    let value = Value::VecF32(vec![3.1415927, 0.001]);
    let string = value.to_string();
    assert_eq!(String::from(r#"[3.1415927,0.001]"#), string)
}

#[test]
fn convert_f64_vec_value_to_string() {
    let value = Value::VecF64(vec![2984191233650.1]);
    let string = value.to_string();
    assert_eq!(String::from(r#"[2984191233650.1]"#), string)
}

#[test]
fn convert_i8_vec_value_to_string() {
    let value = Value::VecI8(vec![-1]);
    let string = value.to_string();
    assert_eq!(String::from(r#"[-1]"#), string)
}

#[test]
fn convert_isize_vec_value_to_string() {
    let value = Value::VecISize(vec![-1,2,-3,4,-5]);
    let string = value.to_string();
    assert_eq!(String::from(r#"[-1,2,-3,4,-5]"#), string)
}

#[test]
fn convert_u8_vec_value_to_string() {
    let value = Value::VecU8(vec![0,1,0,1]);
    let string = value.to_string();
    assert_eq!(String::from(r#"[0,1,0,1]"#), string)
}

#[test]
fn convert_usize_vec_value_to_string() {
    let value = Value::VecUSize(vec![0]);
    let string = value.to_string();
    assert_eq!(String::from(r#"[0]"#), string)
}
