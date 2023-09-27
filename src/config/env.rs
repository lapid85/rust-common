use dotenv;

macro_rules! get_as_type {
    ($fn_name: ident, $type: ty) => {
        pub fn $fn_name(item: &'static str, default_value: $type) -> $type {
            if let Ok(value) = dotenv::var(item) {
                return value.parse::<$type>().unwrap_or(default_value);
            }
            default_value
        }
    };
}

macro_rules! get_as_vec {
    ($fn_name: ident, $type: ty) => {
        pub fn $fn_name(item: &'static str, default_value: Vec<$type>) -> Vec<$type> {
            if let Ok(values) = dotenv::var(item) {
                return values.split(',').map(|v| v.trim()).filter_map(|v| v.parse::<$type>().ok()).collect();
            }
            default_value
        }
    };
}

get_as_type!(get_i8, i8);
get_as_type!(get_u8, u8);
get_as_type!(get_i16, i16);
get_as_type!(get_u16, u16);
get_as_type!(get_i32, i32);
get_as_type!(get_u32, u32);
get_as_type!(get_i64, i64);
get_as_type!(get_u64, u64);
get_as_type!(get_i128, i128);
get_as_type!(get_u128, u128);
get_as_type!(get_isize, isize);
get_as_type!(get_usize, usize);
get_as_type!(get_f32, f32);
get_as_type!(get_f64, f64);
get_as_type!(get_bool, bool);
get_as_type!(get_string, String);
get_as_type!(get_char, char);
get_as_type!(get_path_buf, std::path::PathBuf);

get_as_vec!(get_vec_i8, i8);
get_as_vec!(get_vec_u8, u8);
get_as_vec!(get_vec_i16, i16);
get_as_vec!(get_vec_u16, u16);
get_as_vec!(get_vec_i32, i32);
get_as_vec!(get_vec_u32, u32);
get_as_vec!(get_vec_i64, i64);
get_as_vec!(get_vec_u64, u64);
get_as_vec!(get_vec_i128, i128);
get_as_vec!(get_vec_u128, u128);
get_as_vec!(get_vec_isize, isize);
get_as_vec!(get_vec_usize, usize);
get_as_vec!(get_vec_f32, f32);
get_as_vec!(get_vec_f64, f64);
get_as_vec!(get_vec_bool, bool);
get_as_vec!(get_vec_string, String);
get_as_vec!(get_vec_char, char);
get_as_vec!(get_vec_path_buf, std::path::PathBuf);


/// 获得配置内容
pub fn get_var(item: &'static str, default_value: &'static str) -> String {
    dotenv::var(item).unwrap_or(default_value.to_string())
}

/// 获得配置内容
pub fn get_vec(item: &'static str) -> Vec<String> {
    if let Ok(values) = dotenv::var(item) {
        return  values.split(',').map(|v| v.trim()).map(|v| v.to_string()).collect();
    }
    vec![]
}