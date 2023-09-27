-- Description: This script creates a view that lists all tables in the database
drop view if exists pg_list_tables;
create view pg_list_tables as 
select c.relname as table_name, 
    a.attname as field_name, 
    format_type(a.atttypid, a.atttypmod) as field_type,
    a.attnotnull as not_null,
    col_description(a.attrelid, a.attnum) as comment, 
    a.atthasdef as has_default
from pg_class as c, pg_attribute as a 
where a.attrelid = c.oid and a.attnum > 0 and NOT c.relname like 'pg_%' and NOT c.relname like 'sql_%' and c.relkind = 'r';