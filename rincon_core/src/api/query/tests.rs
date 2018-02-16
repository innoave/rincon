
use api::types::Value;
use super::*;

#[test]
fn query_set_string_parameter() {
    let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
    query.set_parameter("name", "simone");

    assert_eq!(Some(&"simone".to_owned()), query.parameter("name"));
}

#[test]
fn query_set_bool_parameter() {
    let mut query = Query::new("FOR u IN users FILTER u.active = @active RETURN u.name");
    query.set_parameter("active", true);

    assert_eq!(Some(&true), query.parameter("active"));
}

#[test]
fn query_set_i8_parameter() {
    let mut query = Query::new("FOR u IN users FILTER u.id = @id RETURN u.name");
    query.set_parameter("id".to_owned(), Value::I8(-1));

    assert_eq!(Some(&-1i8), query.parameter("id"));
}

#[test]
fn query_set_i64_parameter() {
    let mut query = Query::new("FOR u IN users FILTER u.id = @id RETURN u.name");
    query.set_parameter("id", -1828359i64);

    assert_eq!(Some(&-1828359i64), query.parameter("id"));
}

#[test]
fn query_set_vec_of_f32_parameter() {
    let mut query = Query::new("FOR u IN users FILTER u.id in @ids RETURN u.name");
    let ids = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    query.set_parameter("ids", Value::VecF32(ids));

    assert_eq!(Some(&vec![1.1f32, 2.2, 3.3, 4.4, 5.5]), query.parameter("ids"));
}

#[test]
fn query_set_vec_of_u64_parameter() {
    let mut query = Query::new("FOR u IN users FILTER u.id in @ids RETURN u.name");
    let ids: Vec<u64> = vec![1, 2, 3, 4, 5];
    query.set_parameter("ids", ids);

    assert_eq!(Some(&vec![1u64, 2, 3, 4, 5]), query.parameter("ids"));
}
