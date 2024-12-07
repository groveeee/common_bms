/// 代码生成器

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use sql_builder::SqlBuilder;
    use sqlx::{postgres::PgPoolOptions, Connection, FromRow, Pool, Postgres};
    use serde_json::*;
    use uuid::Uuid;
    use std::env;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, FromRow, Debug)]
    struct Columns {
        column_name: String,
        data_type: String,
        is_nullable: String,
        character_maximum_length: Option<i32>,
        comment: String,
    }

    #[tokio::test]
    async fn get_users() {
        dotenv().ok();
        // 配置日志
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("NO SET DATABASE_URL");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&db_url)
            .await
            .unwrap();
        print_rust_struct("character", &pool).await;
    }

    /// 获取表的字段信息打印对应的Rust结构体
    pub async fn print_rust_struct(table_name: &str, db: &Pool<Postgres>) {
        let sql = r#"WITH columns_info AS (
    SELECT
        column_name,
        data_type,
        is_nullable,
        character_maximum_length,
        ordinal_position
    FROM
        information_schema.columns
    WHERE
        table_name = 'character'
    ORDER BY
        ordinal_position
),
comments AS (
    SELECT
        d.objsubid AS column_ordinal,
        d.description AS comment
    FROM
        pg_description d
    JOIN
        pg_attribute a ON a.attrelid = d.objoid AND a.attnum = d.objsubid
    WHERE
        a.attrelid = 'character'::regclass
    AND
        a.attnum > 0
    ORDER BY
        d.objsubid
)
SELECT
    ci.column_name,
    ci.data_type,
    is_nullable,
    ci.character_maximum_length,
    coalesce(c.comment, '') AS comment
FROM
    columns_info ci
LEFT JOIN
    comments c ON ci.ordinal_position = c.column_ordinal;"#;

        let sql = sql.replace("character", table_name);
        let columns = sqlx::query_as::<_, Columns>(&sql)
            .fetch_all(db).await.unwrap();
        // columns.iter().for_each(|column| { println!("{:?}", column); });
        // 结构体的名字为表名的大写首字母
        println!("#[derive(Serialize,Deserialize, FromRow,Debug)]");
        println!("pub struct {}{} {{", table_name.chars().next().unwrap().to_uppercase(), &table_name[1..]);
        columns.iter().for_each(|column| {
            println!("    pub {}: {},", column.column_name, get_rust_type_from_sql_type(&column.data_type, &column.is_nullable));
        });
        println!("}}");
    }
    fn get_rust_type_from_sql_type(sql_type: &str, is_nullable: &str) -> String {
        let rust_type = match sql_type {
            "int" | "integer" => "i32",
            "bigint" => "i64",
            "smallint" => "i16",
            "tinyint" => "i8",
            "boolean" => "bool",
            "float" | "real" => "f32",
            "double precision" => "f64",
            "decimal" | "numeric" => "f64",
            "character varying" | "varchar" => "String",
            "character" | "char" => "String",
            "uuid" => "Uuid",
            _ => "String",
        };
        if is_nullable == "YES" {
            format!("Option<{}>", rust_type)
        } else {
            rust_type.to_string()
        }
    }
}