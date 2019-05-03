use derive_deconstruct::Deconstruct;

// A struct with no fields.
#[derive(Deconstruct, PartialEq, Debug)]
pub struct Foo {}
// A struct with fields.
#[derive(Deconstruct, PartialEq, Debug)]
pub struct Bar {
    pub x: i32,
    pub y: String,
}
// A unit struct.
#[derive(Deconstruct, PartialEq, Debug)]
pub struct Baz;
// A tuple struct
#[derive(Deconstruct, PartialEq, Debug)]
pub struct Tuple(pub i32, pub i32);

#[test]
fn test_empty_struct() {
    let x = Foo {};
    x.deconstruct();
}
#[test]
fn test_simple_struct() {
    let z = Bar { x: 42, y: "Hello".to_owned() };
    let (x, y) = z.deconstruct();
    assert_eq!(x, 42);
    assert_eq!(y, "Hello".to_owned());
}
#[test]
fn test_unit_struct() {
    let x = Baz;
    let z = x.deconstruct();
    assert_eq!(z, ());
}
#[test]
fn test_simple_tuple_struct() {
    let x = Tuple(5, 6);
    let (z, y) = x.deconstruct();
    assert_eq!(z, 5);
    assert_eq!(y, 6);
}


#[derive(Deconstruct, PartialEq, Debug)]
pub struct Intersection<'scene> {
    pub object: &'scene Bar,
    pub normal: Foo,
    pub point: Foo,
    pub t: f64,
}

#[test]
fn test_struct_with_lifetime() {
    let b = Bar { x: 42, y: "Hello".to_owned() };
    let x = Intersection {
        object: &b, 
        normal: Foo {}, 
        point: Foo {},
        t: 42.0,
    };
    let (object, normal, point, t) = x.deconstruct();
    assert_eq!(object, &b);
    assert_eq!(normal, Foo {});
    assert_eq!(point, Foo {});
    assert_eq!(t, 42.0);
}


#[derive(Deconstruct, PartialEq, Debug)]
pub struct Fred {
    pub x: i32,
    pub y: String,
    pub z: Vec<i8>,
}
//
#[test]
fn test_struct_with_values() {
    let f = Fred {
        x: 3,
        y: "Fred".to_owned(),
        z: vec![-42, 42]
    };
    let (x, y, z) = f.deconstruct();
    
    assert_eq!(x, 3);
    assert_eq!(y, "Fred".to_owned());
    assert_eq!(z, vec![-42, 42]);
}