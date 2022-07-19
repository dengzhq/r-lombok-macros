use r_lombok_macros::Accessors;

/// 源功能描述:
/// 版权所有:dengzhq
/// 未经本人许可,不得以任何方式复制或使用本程序任何部分</p>
/// @author: <a href="mailto:dengzq3@xiaopeng.com">dengzhq</a>
#[derive(Debug, Accessors)]
struct Company {
    name: &'static str,
    boss: &'static str,
    number: u32,
    department: Vec<String>,
}

#[test]
fn test_accessors() {
    let mut xp = Company {
        name: "xp",
        boss: "Big Brother",
        number: u32::MAX,
        department: vec!["HR".to_owned(), "Finance".to_owned()],
    };
    println!("xp name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
    xp.set_name("set_name").set_boss("set_boss").set_number(u32::MIN).set_department(vec!["XP-HR".to_owned(), "XP-Finance".to_owned()]);
    println!("xp accessors name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
}