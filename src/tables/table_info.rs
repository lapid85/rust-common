use crate::types::pg::Pool;

/// Table information
#[derive(Debug, sqlx::FromRow)]
pub struct Table {
    pub name: String,
}

/// Field information
#[derive(Debug, sqlx::FromRow)]
pub struct Field {
    pub table_name: String,
    pub field_name: String,
    pub field_type: String,
    pub not_null: bool,
    pub has_default: bool,
    pub comment: Option<String>,
}

/// List tables in the database
pub async fn list_tables(pool: &Pool) -> Vec<Table> {
    let sql = "SELECT distinct(table_name) as name from pg_list_tables";
    sqlx::query_as(sql).fetch_all(pool).await.unwrap()
}

/// List fields of a table
pub async fn list_fields(pool: &Pool, table_name: &str) -> Vec<Field> {
    let sql = format!("SELECT table_name, field_name, field_type, not_null, has_default, comment FROM pg_list_tables WHERE table_name = '{}'", table_name);
    sqlx::query_as(&sql).fetch_all(pool).await.unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // 注意：在运行测试前，请确保有一个有效的 PostgreSQL 数据库连接池，并替换以下数据库连接信息
    const DATABASE_URL: &str = "postgres://ai:qwe123QWE@localhost:5432/im_platform";

    #[tokio::test]
    async fn test_list_tables() {
        // 建立数据库连接池
        let pool = crate::clients::pg::get_pool(DATABASE_URL).await;
        // 调用 list_tables 函数获取表列表
        let tables = list_tables(&pool).await;
        // 检查结果是否为空
        assert!(!tables.is_empty());
        // 打印表列表
        println!("Tables: {:?}", tables);
    }

    #[tokio::test]
    async fn test_list_fields() {
        // 建立数据库连接池
        let pool = crate::clients::pg::get_pool(DATABASE_URL).await;
        // 替换以下表名为实际存在的表名
        let table_name = "admins";
        // 调用 list_fields 函数获取表字段列表
        let fields = list_fields(&pool, table_name).await;
        // 检查结果是否为空
        assert!(!fields.is_empty());
        // 打印字段列表
        println!("Fields for {}: {:?}", table_name, fields);
    }
}
