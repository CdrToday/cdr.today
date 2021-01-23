use crate::Config;

static CREATE_DB_IF_NOT_EXISTS: &str = r#"
IF NOT EXISTS (
        SELECT *
        FROM sys.databases
        WHERE name = '${DATABASE}'
        )
BEGIN
    CREATE DATABASE ${DATABASE}
END
GO

USE ${DATABASE}
GO
"#;

/// create database
pub fn createdb(c: &Config) -> String {
    CREATE_DB_IF_NOT_EXISTS.replace("${DATABASE}", c.pg.name)
}
