use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(serde::Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_index).service(gcd))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

fn calc_gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        calc_gcd(b, a % b)
    }
}

#[post("/gcd")]
async fn gcd(form: web::Form<GcdParameters>) -> impl Responder {
    let response = format!(
        "The greatest common divisor of {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        calc_gcd(form.n, form.m)
    );
    HttpResponse::Ok().content_type("text/html").body(format!(
        r#"
            <title>GCD Calculator</title>
            {response}
            <p><a href="/">Back</a></p>
            "#
    ))
}
