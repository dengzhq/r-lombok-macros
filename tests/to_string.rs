use r_lombok_macros::ToString;

#[derive(ToString)]
struct Company {
    name: &'static str,
    boss: &'static str,
    number: u32,
    department: Vec<String>,
}

#[test]
fn test_to_string() {
    let xp = Company {
        name: "xp",
        boss: "Big Brother",
        number: u32::MAX,
        department: vec!["HR".to_owned(), "Finance".to_owned()],
    };
    println!("xp = {}", xp.to_string());
    println!("xp = {:?}", xp.to_string());
}