use rand::Rng;

/// 汉字角色名称随机生成方法,3-4个字
pub fn generate_name() -> String {
    let mut name = String::new();
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(3..5);
    for _ in 0..len {
        let ch = rng.sample(rand::distributions::Uniform::new('\u{4e00}', '\u{9fa5}'));
        name.push(ch);
    }
    name
}
