use inflector::Inflector;

/// 将复数转换为单数
pub fn singularize(word: &str) -> String {
    let singular = word.to_owned().to_singular();
    singular
}

/// 将蛇形命名转换为帕斯卡命名
pub fn snake_to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// 将表名转换为结构体名
pub fn table_name_to_struct_name(table_name: &str) -> String {
    let struct_name = singularize(table_name);
    snake_to_pascal_case(struct_name.as_str())
}
