// let pool = PgPoolOptions::new().max_connections(5).connect("postgres://postgres:password@localhost/test").await?;
pub type Pool = sqlx::Pool<sqlx::Postgres>;
pub type PoolOptions = sqlx::postgres::PgPoolOptions;
pub type Conn = sqlx::pool::PoolConnection<sqlx::Postgres>;
pub type DriverType = sqlx::Postgres;
pub type Row = sqlx::postgres::PgRow;
pub type Transaction<'t> = sqlx::Transaction<'t, sqlx::Postgres>;
pub type QueryBuilder<'q> = sqlx::query_builder::QueryBuilder<'q, sqlx::Postgres>;
pub type Error = sqlx::Error;
//pub type Result<T> = Result<T, &'static str>;

// 类据类型
#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Str(String),
    Bool(bool),
    F32(f32),
    F64(f64),
    S(&'static str),
}

macro_rules! impl_from_val {
    ($($type:ident => $enum_type: ident),+) => {
        $(
            impl From<$type> for Val {
                fn from(v: $type) -> Self {
                    Val::$enum_type(v)
                }
            }
        )+
    };
}

impl From<&'static str> for Val {
    fn from(v: &'static str) -> Self {
        crate::types::Val::S(v)
    }
}

macro_rules! impl_into_val {
    ($($type:ident),+) => {
        $(
            impl Into<$type> for Val {
                fn into(self) -> $type {
                    match self {
                        crate::types::Val::I8(v) => v as $type,
                        crate::types::Val::U8(v) => v as $type,
                        crate::types::Val::I16(v) => v as $type,
                        crate::types::Val::U16(v) => v as $type,
                        crate::types::Val::I32(v) => v as $type,
                        crate::types::Val::U32(v) => v as $type,
                        crate::types::Val::I64(v) => v as $type,
                        crate::types::Val::U64(v) => v as $type,
                        crate::types::Val::F32(v) => v as $type,
                        crate::types::Val::F64(v) => v as $type,
                        v => panic!("can not convert {:?} to {}", v, stringify!($type)),
                    }
                }
            }
        )+
    };
}

impl Into<&'static str> for Val {
    fn into(self) -> &'static str {
        match self {
            crate::types::Val::S(v) => v,
            _ => "",
        }
    }
}
impl Into<String> for Val {
    fn into(self) -> String {
        match self {
            crate::types::Val::Str(v) => v,
            _ => "".to_string(),
        }
    }
}
impl Into<bool> for Val {
    fn into(self) -> bool {
        match self {
            crate::types::Val::Bool(v) => v,
            _ => false,
        }
    }
}

impl_from_val![
    i8 => I8,
    u8 => U8,
    i16 => I16,
    u16 => U16,
    i32 => I32,
    u32 => U32,
    i64 => I64,
    u64 => U64,
    String => Str,
    bool => Bool,
    f32 => F32,
    f64 => F64
];

impl_into_val![i8, u8, i16, u16, i32, u32, i64, u64, f32, f64];

#[derive(sqlx::FromRow, Debug)]
pub struct Total {
    pub total: i64,
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_from_i8() {
        let val: crate::types::Val = 42i8.into();
        assert_eq!(val, crate::types::Val::I8(42));
    }

    #[test]
    fn test_into_i8() {
        let val = crate::types::Val::I8(42);
        let result: i8 = val.into();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_from_str() {
        let val = crate::types::Val::from("hello");
        assert_eq!(val, crate::types::Val::S("hello"));
    }

    #[test]
    fn test_into_string() {
        let val = crate::types::Val::Str("world".to_string());
        let result: String = val.into();
        assert_eq!(result, "world");
    }

    #[test]
    fn test_into_string_from_string() {
        let val = crate::types::Val::Str("rust".to_string());
        let result: String = val.into();
        assert_eq!(result, "rust");
    }
}
