use super::Val;

#[derive(Debug)]
pub struct Cond {
    fields: Vec<String>, // 存储 AND 条件
    operators: Vec<String>, // 存储操作符
    page_size: Option<i32>, // 存储 LIMIT 条件
    page_offset: Option<i32>,  // 存储分页条件
    order_sort: Option<String>, // 存储 ORDER BY 子句
    pub args: Vec<Val>, // 存储参数
    arg_count: i32, // 存储参数个数
}

impl Cond {
    pub fn new() -> Self {
        Cond {
            fields: vec![], // 存储 AND 条件
            operators: vec![], // 存储操作符
            args : vec![], // 存储参数
            page_size: None, // 存储 LIMIT 条件
            page_offset: None,  // 存储分页条件
            order_sort: None, // 存储 ORDER BY 子句
            arg_count: 0, // 存储参数个数
        }
    }

    /// 判断是否空参数
    pub fn has_args(&self) -> bool {
        self.arg_count > 0
    }

    /// 用于构建 WHERE 子句
    pub fn and(&mut self, condition: &str) -> &mut Self {
        self.fields.push(condition.to_string());
        self
    }

    /// 用于构建 WHERE 子句
    pub fn or(&mut self, condition: &str) -> &mut Self {
        self.fields.push(condition.to_string());
        self
    }

    /// 用于 = 查询
    pub fn eq(&mut self, column: &str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push("=".to_owned());
        self.args.push(value.clone());
        self
    }

    /// 用于 >= 查询
    pub fn ge(&mut self, column: &str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push(">=".to_owned());
        self.args.push(value.clone());
        self
    }

    /// 用于 <= 查询
    pub fn le(&mut self, column: &str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push("<=".to_owned());
        self.args.push(value.clone());
        self
    }

    /// 用于 > 查询
    pub fn gt(&mut self, column: &str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push(">".to_owned());
        self.args.push(value.clone());
        self
    }

    /// 用于 < 查询
    pub fn lt(&mut self, column: &str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push("<".to_owned());
        self.args.push(value.clone());
        self
    }

    /// 用于 BETWEEN 查询
    pub fn between(&mut self, column: &str, min: &Val, max: &Val) -> &mut Self {
        self.arg_count += 2 ;
        self.fields.push(column.to_owned());
        self.operators.push(">".to_owned());
        self.args.push(min.clone());
        self.fields.push(column.to_owned());
        self.operators.push("<".to_owned());
        self.args.push(max.clone());
        self
    }

    /// 用于模糊查询
    pub fn like(&mut self, column: &str, pattern: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push("LIKE".to_owned());
        self.args.push(pattern.clone());
        self
    }

    /// 用于 limit 查询
    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.page_size = Some(limit);
        self
    }

    /// 用于 order by 查询
    pub fn order_by(&mut self, column: &str, ascending: bool) -> &mut Self {
        let order = if ascending { "ASC" } else { "DESC" };
        self.order_sort = Some(format!("ORDER BY {} {}", column, order));
        self
    }

    /// 用于分页查询
    pub fn page(&mut self, page: i32) -> &mut Self {
        self.page_offset = Some(page);
        self
    }

    /// 用于构建查询语句
    pub fn build(&self) -> String {
        let mut query = String::new();
        let has_and = self.fields.len() > 0;
        if has_and {
            for (i, field) in self.fields.iter().enumerate() {
                let operator = &self.operators[i as usize];
                let sql = format!("{} {} ${}", field, operator, i + 1);
                query.push_str(&sql);
            }
        }
        if let Some(order_by) = &self.order_sort {
            query.push_str(&format!(" {}", order_by));
        }
        if let Some(limit) = self.page_size {
            query.push_str(&format!(" LIMIT {}", limit));
            if let Some(page) = self.page_offset {
                let offset = (page - 1) * limit; // 默认一页10条记录
                query.push_str(&format!(" OFFSET {}", offset));
            }
        }

        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_or() {
        let query = Cond::new()
            .and("age >= 18")
            .or("city = 'New York'")
            .or("city = 'London'")
            .build();
        assert_eq!(query, "WHERE age >= 18 AND (city = 'New York' OR city = 'London')");
    }

    #[test]
    fn test_build_limit_page() {
        let query = Cond::new().limit(10).page(2).build();
        assert_eq!(query, " LIMIT 10 OFFSET 10");
    }

    // 添加更多测试用例来测试其他方法
}
