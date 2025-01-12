use super::*;
use log::info;
use rusqlite::params;
use std::io::Write;
use std::sync::Once;

static INIT: Once = Once::new();

fn init_logger() {
    // INIT.call_once(|| {
    //     env_logger::init();
    // });
}

macro_rules! debug_println {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
        std::io::stdout().flush().unwrap();
    })
}

#[cfg(test)]
mod tests {
    // 导入父模块的所有项，使得测试模块可以访问它们
    use super::*;
    // 一个简单的测试函数
    #[test]
    fn test_add() {
        init_logger();
        debug_println!("aaa{}", crate::util::env::get_logs_dir());

        let result1 = sqlite_exec(
            "insert into t_name(name) values(?)",
            Some(vec![Value::String("John Doe".to_string())]),
        );
        //let result = query("select * from abc",Vec::new());
        match result1 {
            Ok(rows) => info!("{}", rows),
            Err(e) => info!("{}", e.msg),
        }
        println!("test_add");
    }
    #[test]
    fn test_add_negative() -> Result<(), Box<dyn std::error::Error>> {
        crate::log::init_log4rs();
        crate::db::load_config();

        let conn = crate::db::sqlite::get_conn()?;

        let multi_line_text = r#"

CREATE TABLE "main"."无标题" (
  "e" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
  "f" text,
  "g" text,
  "x" integer
);

CREATE INDEX "main"."idx_test"
ON "无标题" (
  "f"
);


CREATE TABLE "yyy" (
  "a" TEXT NOT NULL,
  "b" TEXT,
  "e" text,
  PRIMARY KEY ("a")
);

create table "yyy" (
  "a" TEXT NOT NULL,
  "b" TEXT,
  "e" text,
  PRIMARY KEY ("a")
);
CREATE INDEX "main"."yyyx"
ON "yyy" (
  "b"
);
        "#;
        let _x = r#"
        CREATE INDEX "f"
ON "main"."无标题" (
  "f"
);

CREATE INDEX "main"."f"
ON "无标题" (
  "f"
);

CREATE INDEX "f"
ON "无标题" (
  "f"
);
        
        "#;

        crate::db::migrate::update_database_structure(&conn, multi_line_text)?;
        Ok(())
    }

    #[test]
    fn xx() -> Result<(), Box<dyn std::error::Error>> {
        crate::log::init_log4rs();
        crate::db::load_config();

        // let conn = crate::db::sqlite::get_conn()?;

        let rows = sqlite_query("PRAGMA compile_options;", None);

        println!("{:?}", rows);
        Ok(())
    }
}
