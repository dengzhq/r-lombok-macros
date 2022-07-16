use std::fmt::Debug;

use r_lombok_macros::{Getter, Setter};

/// 源功能描述: show origin code： cargo expand --test=getter --color=always
/// 版权所有:dengzhq
/// 未经本人许可,不得以任何方式复制或使用本程序任何部分</p>
/// @author: <a href="mailto:dengzq3@xiaopeng.com">dengzhq</a>
#[derive(Debug, Getter, Setter)]
struct Company {
    name: &'static str,
    boss: &'static str,
    number: u32,
    department: Vec<String>,
}

#[derive(Getter, Setter)]
struct CompanyGen<T> where T: Debug {
    name: T,
    boss: &'static str,
    number: u32,
    department: Vec<String>,
}

#[derive(Getter, Setter)]
struct CompanyWrap {
    sub_company: CompanyGen<Company>,
    name: &'static str,
}

// Unit struct
#[derive(Getter, Setter, Debug)]
struct UnitStruct {}

// Tuple struct：panic
// #[derive(Setter)]
// struct TupleStruct(i32, String);

#[test]
fn test_getter_setter() {
    let mut xp = Company {
        name: "xp",
        boss: "Big Brother",
        number: u32::MAX,
        department: vec!["HR".to_owned(), "Finance".to_owned()],
    };
    println!("xp name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
    xp.set_name("set_name");
    xp.set_boss("set_boss");
    xp.set_number(u32::MIN);
    xp.set_department(vec!["department".to_owned()]);


    let xp_t = CompanyGen::<Company> {
        name: xp,
        boss: "Big Brother",
        number: u32::MAX,
        department: vec!["HR".to_owned(), "Finance".to_owned()],
    };
    println!("xp_t name = {:?} boss = {:?} number = {:?} department = {:?}", xp_t.get_name(), xp_t.get_boss(), xp_t.get_number(), xp_t.get_department());

    let xp_wrap = CompanyWrap {
        sub_company: xp_t,
        name: "xp_wrap",
    };
    println!("xp_wrap name = {:?} sub_company = {:?}", xp_wrap.get_name(), xp_wrap.get_sub_company().get_name());
    let unit = UnitStruct {};
    println!("unit = {:?}", unit);
}
