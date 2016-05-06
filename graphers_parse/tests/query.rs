extern crate graphers_core as core;
extern crate graphers_parse as parse;

use core::*;

#[test]
fn test_basic_query() {
    let context = parse::parse("
        query {
            person(id: \"1\") {
                first_name,
                zuck: friend(name: \"Mark Zuckerberg\")
            }
        }
    ");

    let query = context.query().expect("there should be a query");

    let person_field = query.fields().get(0).expect("should have a person field");

    let subquery = person_field.subquery().expect("should have subquery");
    let first_name_field = subquery.fields().get(0).expect("should have a first name field");
    let friend_field = subquery.fields().get(1).expect("should have a friend field");

    assert_eq!(person_field.name().as_str(), "person");
    assert_eq!(first_name_field.name().as_str(), "first_name");
    assert_eq!(friend_field.name().as_str(), "friend");

    assert_eq!(person_field.alias().as_str(), "person");
    assert_eq!(first_name_field.alias().as_str(), "first_name");
    assert_eq!(friend_field.alias().as_str(), "zuck");

    assert_eq!(person_field.arguments()[0].name().as_str(), "id");
    assert_eq!(person_field.arguments()[0].value(), &query::Value::String("1".into()));
    assert_eq!(first_name_field.arguments().len(), 0);
    assert_eq!(friend_field.arguments()[0].name().as_str(), "name");
    assert_eq!(friend_field.arguments()[0].value(), &query::Value::String("Mark Zuckerberg".into()));
}