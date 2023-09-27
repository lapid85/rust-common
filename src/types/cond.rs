pub struct Cond {
    conditions: Vec<String>, // 存储 AND 条件
    or_conditions: Vec<String>, // 存储 OR 条件
    page_size: Option<usize>, // 存储 LIMIT 条件
    page_offset: Option<usize>,  // 存储分页条件
    order_sort: Option<String>, // 存储 ORDER BY 子句
}

impl Cond {
    pub fn new() -> Self {
        Cond {
            conditions: vec![], // 存储 AND 条件
            or_conditions: vec![], // 存储 OR 条件
            page_size: None, // 存储 LIMIT 条件
            page_offset: None,  // 存储分页条件
            order_sort: None, // 存储 ORDER BY 子句
        }
    }

    fn where_clause(&mut self, condition: &str) -> &mut Self {
        self.conditions.push(condition.to_string());
        self
    }

    pub fn and(&mut self, condition: &str) -> &mut Self {
        self.conditions.push(condition.to_string());
        self
    }

    pub fn or(&mut self, condition: &str) -> &mut Self {
        self.or_conditions.push(condition.to_string());
        self
    }

    pub fn eq(&mut self, column: &str, value: &str) -> &mut Self {
        let condition = format!("{} = '{}'", column, value);
        self.where_clause(&condition)
    }

    pub fn ge(&mut self, column: &str, value: &str) -> &mut Self {
        let condition = format!("{} >= '{}'", column, value);
        self.where_clause(&condition)
    }

    pub fn le(&mut self, column: &str, value: &str) -> &mut Self {
        let condition = format!("{} <= '{}'", column, value);
        self.where_clause(&condition)
    }

    pub fn gt(&mut self, column: &str, value: &str) -> &mut Self {
        let condition = format!("{} > '{}'", column, value);
        self.where_clause(&condition)
    }

    pub fn lt(&mut self, column: &str, value: &str) -> &mut Self {
        let condition = format!("{} < '{}'", column, value);
        self.where_clause(&condition)
    }

    pub fn between(&mut self, column: &str, min: &str, max: &str) -> &mut Self {
        let condition = format!("{} BETWEEN '{}' AND '{}'", column, min, max);
        self.where_clause(&condition)
    }

    pub fn like(&mut self, column: &str, pattern: &str) -> &mut Self {
        let condition = format!("{} LIKE '{}'", column, pattern);
        self.where_clause(&condition)
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.page_size = Some(limit);
        self
    }

    pub fn order_by(&mut self, column: &str, ascending: bool) -> &mut Self {
        let order = if ascending { "ASC" } else { "DESC" };
        self.order_sort = Some(format!("ORDER BY {} {}", column, order));
        self
    }

    pub fn page(&mut self, page: usize) -> &mut Self {
        self.page_offset = Some(page);
        self
    }

    pub fn build(&self) -> String {
        let mut query = String::new();
        let has_and = !self.conditions.is_empty();
        let has_or = !self.or_conditions.is_empty();
        if has_and || has_or {
            query.push_str("WHERE ");
        }
        if has_and {
            query.push_str(&self.conditions.join(" AND "));
        }
        if has_or {
            query.push_str(" AND (");
            query.push_str(&self.or_conditions.join(" OR "));
            query.push_str(")");
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
            .where_clause("age >= 18")
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
