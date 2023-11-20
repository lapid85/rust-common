use super::Val;
use crate::consts;
use std::string::ToString;

#[derive(Debug)]
pub struct Cond {
    pub fields: Vec<String>, // 存储 AND 条件
    pub operators: Vec<String>, // 存储操作符
    page_size: Option<i32>, // 存储 LIMIT 条件
    page: Option<i32>,  // 存储分页条件
    order_sort: Option<String>, // 存储 ORDER BY 子句
    pub args: Vec<Val>, // 存储参数
    pub arg_count: i32, // 存储参数个数
}

impl Cond {
    pub fn new() -> Self {
        Cond {
            fields: vec![], // 存储 AND 条件
            operators: vec![], // 存储操作符
            args : vec![], // 存储参数
            page_size: None, // 存储 LIMIT 条件
            page: None,  // 存储分页条件
            order_sort: None, // 存储 ORDER BY 子句
            arg_count: 0, // 存储参数个数
        }
    }

    /// 从数组构建
    pub fn from_array(array: &[(&'static str, &'static str, Val)]) -> Self {
        let mut cond = Cond::new();
        for (field, operator, value) in array.iter() {
            match operator {
                &"=" => { cond.eq(field, value); },
                &">=" => { cond.ge(field, value); },
                &"<=" => { cond.le(field, value); },
                &">" => { cond.gt(field, value); },
                &"<" => { cond.lt(field, value); },
                &"LIKE" => { cond.like(field, value); },
                _ => { cond.eq(field, value); },
            }
        }
        cond
    }

    /// 判断是否空参数
    pub fn has_args(&self) -> bool {
        self.arg_count > 0
    }

    /// 组合多个条件
    fn add_cond(&mut self, condition: &Cond, opera: &'static str) -> &mut Self {
        let field_count = condition.fields.len();
        if field_count == 0 { // 如果没有参数，直接返回
            return self
        }
        let field_max = field_count - 1;
        for i in 0..field_count {
            let field = &condition.fields[i];
            let operation = &condition.operators[i];
            let op_arr = operation.split(" ").collect::<Vec<&str>>();
            self.arg_count += 1;
            if i == 0 && field_max == 0 {
                self.fields.push(format!("{} ({}", opera, field));
                self.operators.push(format!("{} ${})", op_arr[0], self.arg_count));
            } else if i == 0 && field_max > 0 {
                self.fields.push(format!("{} ({}", opera, field));
                self.operators.push(format!("{} ${}", op_arr[0], self.arg_count));
            } else if i > 0 && i == field_max {
                self.fields.push(field.to_owned());
                self.operators.push(format!("{} ${})", op_arr[0], self.arg_count));
            } else if i > 0 && i < field_max {
                self.fields.push(field.to_owned());
                self.operators.push(format!("{} ${}", op_arr[0], self.arg_count));
            }
            let arg = &condition.args[i];
            self.args.push(arg.clone());
        }
        self
    }

    /// 用于构建 WHERE 子句
    pub fn and(&mut self, condition: &Cond) -> &mut Self {
        self.add_cond(condition, "AND")
    }

    /// 用于构建 WHERE 子句
    pub fn or(&mut self, condition: &Cond) -> &mut Self {
        self.add_cond(condition, "OR")
    }

    /// 用于 = 查询
    pub fn eq(&mut self, column: &'static str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push(format!("= ${}", self.arg_count));
        self.args.push(value.clone());
        self
    }

    /// 用于 >= 查询
    pub fn ge(&mut self, column: &'static str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push(format!(">= ${}", self.arg_count));
        self.args.push(value.clone());
        self
    }

    /// 用于 <= 查询
    pub fn le(&mut self, column: &'static str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push(format!("<= ${}", self.arg_count));
        self.args.push(value.clone());
        self
    }

    /// 用于 > 查询
    pub fn gt(&mut self, column: &'static str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push(format!("> ${}", self.arg_count));
        self.args.push(value.clone());
        self
    }

    /// 用于 < 查询
    pub fn lt(&mut self, column: &'static str, value: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push(format!("< ${}", self.arg_count));
        self.args.push(value.clone());
        self
    }

    /// 用于 BETWEEN 查询
    pub fn between(&mut self, column: &'static str, min: &Val, max: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(format!("({}", column));
        self.operators.push(format!("> ${}", self.arg_count));
        self.args.push(min.clone());

        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push(format!("< ${})", self.arg_count));
        self.args.push(max.clone());
        self
    }

    /// 用于模糊查询
    pub fn like(&mut self, column: &'static str, pattern: &Val) -> &mut Self {
        self.arg_count += 1 ;
        self.fields.push(column.to_owned());
        self.operators.push(format!("LIKE ${}", self.arg_count));
        self.args.push(pattern.clone());
        self
    }

    /// 用于 limit 查询
    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.page_size = Some(limit);
        self
    }

    /// 用于 order by 查询
    pub fn order_by(&mut self, column_order: &str) -> &mut Self {
        self.order_sort = Some(format!("{}", column_order));
        self
    }

    /// 用于分页查询
    pub fn page(&mut self, page: i32) -> &mut Self {
        self.page= Some(page);
        self
    }

    /// 用于构建查询语句
    pub fn build(&self) -> String {
        let mut query = String::new();
        let field_count = self.fields.len();
        let has_and = field_count > 0;
        if has_and {
            for i in 0..field_count {
                let operator = &self.operators[i]; // 操作符
                let field = &self.fields[i]; // 字段
                if i > 0  {
                    if !field.starts_with("AND") && !field.starts_with("OR") {
                        query.push_str(" AND ");
                    } else {
                        query.push_str(" ");
                    }
                }                 
                let sql = format!("{} {}", field, operator);
                query.push_str(&sql);
            }
        }

        query
    }

    /// 获取排序信息
    pub fn get_order_by(&self) -> Option<String> {
        self.order_sort.clone()
    }

    /// 获取分页信息 page, page_size
    pub fn get_limits(&self) -> (i32, i32) {
        let mut page_size = consts::PAGE_SIZE;
        let mut page = consts::PAGE_DEFAULT;

        if let Some(limit) = self.page_size {
            page_size = limit;
            if let Some(page_current) = self.page{
                page = page_current;
            }
        }

        (page, page_size)
    }
}

/// 实现 ToString trait
impl ToString for Cond {
    fn to_string(&self) -> String {
        self.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_limit_page() {
        let query = Cond::new().limit(10).page(2).build();
        assert_eq!(query, " LIMIT 10 OFFSET 10");
    }
}
