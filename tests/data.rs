use r_lombok_macros::Data;

/// 源功能描述:
/// 版权所有:dengzhq
/// 未经本人许可,不得以任何方式复制或使用本程序任何部分</p>
/// @author: <a href="mailto:690594922@qq.com">dengzhq</a>
#[derive(Debug, Data)]
struct Company {
    name: &'static str,
    boss: &'static str,
    number: u32,
    department: Vec<String>,
}

#[test]
fn test_data() {
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
    xp.set_department(vec!["XP-HR".to_owned(), "XP-Finance".to_owned()]);
    println!("xp data name = {:?} boss = {:?} number = {:?} department = {:?}", xp.get_name(), xp.get_boss(), xp.get_number(), xp.get_department());
}