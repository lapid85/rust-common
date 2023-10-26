use crate::tables::table_info;
use crate::types::pg::Pool;
use crate::utils::strings;
use std::fs;

/// 获取结构体字段类型
pub fn get_struct_field_type(table_name: &String, field_type: &str) -> &'static str {
    match field_type {
        "smallint" => "i16",
        "integer" => "i32",
        "bigint" => "i64",
        "text" => "String",
        "bool" => "bool",
        "date" => "chrono::NaiveDate",
        v => {
            if v.contains("character varying") {
                "String"
            } else if v.contains("timestamp without time zone") {
                "chrono::NaiveDateTime"
            } else if v.contains("timestamp with time zone") {
                "chrono::DateTime<chrono::Utc>"
            } else if v.contains("decimal") { 
                "f64"
            } else if v.contains("numeric") { 
                "rust_decimal::Decimal"
            } else {
                panic!("未知类型: {} - {}", table_name, v);
            }
        }
    }
}

/// 创建结构体
pub async fn create_tables(pool: &Pool) {
    let tables = table_info::list_tables(pool).await;
    let mut structs = String::new();

    let Ok(mut current_dir) = std::env::current_dir() else {
        panic!("无法获取当前目录")
    };
    current_dir.push("src/models");

    // 如果不存在, 则创建目录
    if !current_dir.exists() {
        let Ok(()) = std::fs::create_dir(&current_dir) else {
            panic!("无法创建目录: {}", current_dir.to_str().unwrap());
        };
    }

    // 创建结构体
    for table in tables {
        let mut struct_str = String::new();
        // //!   */
        // use crud_derive::CRUDTable;
        // use serde::{Deserialize, Serialize};
        // use sqlx::FromRow;
        // use std::fmt::Debug;
        // #[derive(Serialize, Deserialize, Debug, FromRow, Default, CRUDTable, Clone)]
        struct_str.push_str(&format!("//! 表名: {} - 本文档为脚本自动生成,每次生成都会被自动覆盖,请不要修改 \n\n", table.name.as_str()));
        struct_str.push_str("use crud_derive::CRUDTable;\n");
        struct_str.push_str("use serde::{Deserialize, Serialize};\n");
        struct_str.push_str("use sqlx::FromRow;\n");
        struct_str.push_str("use std::fmt::Debug;\n");
        struct_str.push_str("\n");

        let struct_name = strings::table_name_to_struct_name(table.name.as_str());
        struct_str.push_str(&format!("/// 模型: {}\n", struct_name));
        struct_str.push_str(
            "#[derive(Serialize, Deserialize, Debug, FromRow, Default, CRUDTable, Clone)]\n",
        );
        struct_str.push_str(&format!("pub struct {} {{\n", struct_name));

        let fields = table_info::list_fields(pool, &table.name).await;
        for field in fields {
            let field_type = get_struct_field_type(&table.name, field.field_type.as_str());
            let field_name = field.field_name.to_lowercase();
            let field_comment = field.comment.unwrap_or("".to_string());
            if field_type.contains("rust_decimal") {
                struct_str.push_str("    /// 参考 [`rust_decimal::Decimal`]\n");
            } else if field.field_type == "date" { 
                struct_str.push_str("    /// 参考 [`chrono::NaiveDate`]\n");
            }
            struct_str.push_str(&format!("    pub {}: {}, // {} - {}\n", field_name, field_type, field_comment, field.field_type));
        }
        struct_str.push_str("}\n");
        structs.push_str(&format!("pub mod {};\n", table.name)); // 创建 mod.rs
        structs.push_str(&format!("pub use {}::*;\n", table.name)); // 创建 mod.rs

        let mut file_path = current_dir.clone();
        file_path.push(format!("{}.rs", table.name));
        fs::write(file_path, &struct_str).expect("Unable to write file");

        println!("====>> creating struct: {}", table.name);
        println!("{}\n", struct_str);
    }

    // 创建 mod.rs
    let mut mod_file = current_dir.clone();
    mod_file.push("mod.rs");
    fs::write(mod_file, structs).expect("Unable to write file");
}
