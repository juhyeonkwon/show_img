use actix_files as fs;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

/*
//requset를 handle 합니다..
fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}
*/

//여기서 라우트를 합니다..
fn main() {
     HttpServer::new(|| {
     App::new()
        .service(fs::Files::new("/", "./static/").index_file("img.jpg"),
        )
    })   
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}

