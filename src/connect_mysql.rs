use mysql::*;
use mysql::prelude::*;

fn connect_mysql() -> Result<(), Box<dyn std::error::Error>> {
    // 数据库连接 URL: mysql://user:password@host:port/database
    let url = "mysql://root:password@localhost:3306/test";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // 创建表
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name TEXT NOT NULL
        )",
    )?;

    // 插入数据
    conn.exec_drop("INSERT INTO users (name) VALUES (?)", ("Alice",))?;

    // 查询数据
    let users: Vec<String> = conn.query("SELECT name FROM users")?;
    for name in users {
        println!("User: {}", name);
    }

    Ok(())
}

