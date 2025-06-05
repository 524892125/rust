use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;




#[derive(Deserialize)]
struct UserQuery {
    name: String,
}


#[post("/find_user")]
async fn find_user(req: web::Json<UserQuery>) -> impl Responder {
    match find_user_by_name(&req.name) {
        Ok(users) if !users.is_empty() => {
            HttpResponse::Ok().json(json!({ "users": users }))
        }
        Ok(_) => HttpResponse::NotFound().body("No users found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB Error: {}", e)),
    }
}


fn find_user_by_name(name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = "mysql://root:123456@192.168.9.128:3306/kiif_test01";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // 查询匹配名字的用户
    let users: Vec<String> = conn.exec("SELECT name FROM users WHERE name = ?", (name,))?;
    Ok(users)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8085");

    HttpServer::new(|| {
        App::new()
            .service(find_user) // 注册 POST 接口
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}