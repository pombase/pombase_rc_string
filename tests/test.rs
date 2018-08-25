#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate pombase_rc_string;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct SerTest {
    field: RcString,
}

#[derive(Serialize, Deserialize)]
struct SerMapTestStruct {
    field1: RcString,
    field2: Option<RcString>,
}

#[derive(Serialize, Deserialize)]
struct SerMapTest {
    field: HashMap<RcString, SerMapTestStruct>,
}

use pombase_rc_string::RcString;

#[test]
fn test() {
    let s = RcString::from("test");
    assert!(s == "test");
    assert_eq!(s.ref_count(), 1);
    assert_eq!(s.to_string(), "test");
    let s1 = s.clone();
    assert_eq!(s.ref_count(), 2);
    assert_eq!(s1.to_string(), "test");

    {
        let s2 = s.clone();
        assert_eq!(s2.ref_count(), 3);
    }

    assert_eq!(s.ref_count(), 2);

    let s3: &str = &s;
    assert_eq!(s3, "test");

    let mut m = SerMapTest {
        field: HashMap::new()
    };

    let st = SerMapTestStruct {
        field1: RcString::from("field1_value"),
        field2: Some(RcString::from("field1_value")),
    };

    m.field.insert("key".into(), st);

    for key in m.field.keys() {
        assert_eq!(key, "key");
    }
    for v in m.field.values() {
        assert_eq!(v.field1, "field1_value");
    }

    let serialized = serde_json::to_string(&m).unwrap();

    assert_eq!(serialized, "{\"field\":{\"key\":{\"field1\":\"field1_value\",\"field2\":\"field1_value\"}}}");

    let deserialized_map: SerMapTest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized_map.field.get(&RcString::from("key")).unwrap().field1,
               "field1_value");

    let ser_test = SerTest { field: RcString::from("field_value") };

    let serialized = serde_json::to_string(&ser_test).unwrap();

    assert_eq!(serialized, "{\"field\":\"field_value\"}");

    let deserialized: SerTest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.field, "field_value");
}

