-- 列表所有数据表
drop view if exists all_tables;
create view all_tables as 
select c.relname as table_name, 
    a.attname as field_name, 
    format_type(a.atttypid, a.atttypmod) as field_type,
    a.attnotnull as not_null,
    col_description(a.attrelid, a.attnum) as comment, 
    a.atthasdef as has_default
from pg_class as c, pg_attribute as a 
where a.attrelid = c.oid 
    and a.attnum > 0 
    and NOT c.relname like 'pg_%' 
    and NOT c.relname like 'sql_%' 
    and NOT c.relname like 'all_table%'
    and c.relkind = 'r';