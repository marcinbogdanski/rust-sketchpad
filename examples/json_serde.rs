

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Person {
    name: String,
    age: usize,
    verified: bool,
}

fn main() {

    let person = Person {
        name: "Adam".to_string(),
        age: 33,
        verified: true,
    };
    println!("{:?}", person);

    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);

    println!("-------------------");
    let json = r#"
        {
          "name": "George",
          "age": 27,
          "verified": false
        }
    "#;

    let person: Person = serde_json::from_str(json).unwrap();

    println!("{:?}", person);
}
