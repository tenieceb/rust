pub mod journal;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_web::middleware::Logger;
use tera::{Tera, Context};
use serde::Deserialize;
use mood_web::journal::entry::EmotionEntry;
use mood_web::{load_entries, save_or_update_today_entry};

#[derive(Deserialize)]
struct MoodForm {
    date: String,
    emotion: String,
    note: Option<String>,
}

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let s = tmpl.render("index.html.tera", &Context::new())
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn submit(
    form: web::Form<MoodForm>,
) -> Result<HttpResponse> {
    let entry = EmotionEntry::new(
        form.date.clone(),
        form.emotion.clone(),
        form.note.clone(),
    );

    save_or_update_today_entry("moods.csv", entry)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to save mood"))?;

    // Redirect to list page after submission
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/list"))
        .finish())
}

async fn list(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let entries = load_entries("moods.csv").unwrap_or_default();

    let mut context = Context::new();
    context.insert("entries", &entries);

    let s = tmpl.render("list.html.tera", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn blanket(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let entries = load_entries("moods.csv").unwrap_or_default();

    let mut context = Context::new();
    context.insert("entries", &entries);

    let s = tmpl.render("blanket.html.tera", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger (optional)
    env_logger::init();

    // Compile templates from ./templates folder
    let tera = Tera::new("templates/**/*").unwrap();

    // URL to open
    let url = "http://127.0.0.1:8080/";

    // Open in default browser (non-blocking)
    if webbrowser::open(url).is_ok() {
        println!("Opened {} in your default browser", url);
    }
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .route("/submit", web::post().to(submit))
            .route("/list", web::get().to(list))
            .route("/blanket", web::get().to(blanket))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

