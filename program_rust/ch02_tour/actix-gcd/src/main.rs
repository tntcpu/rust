use actix_web::{web, App, HttpResponse, HttpServer};

fn main() {
    let server = HttpServer::new(|| App::new().route("/", web::get().to(get_index)));

    println!("Servering on http://localhost:7878...");
    server
        .bind("127.0.0.1:7878")
        .expect("Can not bind to port 7878")
        .run()
        .expect("Can not run server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"
                <title>GCD Calculator</title>
                <from action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#
        )
}
