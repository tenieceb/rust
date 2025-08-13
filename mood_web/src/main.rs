pub mod journal;
use actix_files as fs;   
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_web::middleware::Logger;
use actix_web::error::ErrorBadRequest; 
use chrono::Local;
use tera::{Tera, Context};
use serde::Deserialize;
use mood_web::journal::entry::EmotionEntry;
use mood_web::{load_entries, save_or_update_today_entry};
use std::io::Write;

#[derive(Deserialize)]
struct MoodForm {
    date: String,
    emotion: String,
    other_emotion: Option<String>,
    note: Option<String>,
}

#[derive(Deserialize)]
struct DeleteForm {
    date: String,
}


async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut ctx = Context::new();
    let today = Local::now().format("%Y-%m-%d").to_string();
    ctx.insert("today", &today);

    let s = tmpl
        .render("index.html.tera", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn submit(form: web::Form<MoodForm>) -> Result<HttpResponse> {
    // Decide the actual emotion to store
    let emotion = if form.emotion.trim().eq_ignore_ascii_case("other") {
        let custom = form.other_emotion.as_deref().unwrap_or("").trim();
        if custom.is_empty() {
            return Err(ErrorBadRequest("Please enter your emotion for 'Other'."));
        }
        custom.to_string()
    } else {
        form.emotion.trim().to_string()
    };

    let emotion = emotion.to_lowercase();

    let entry = EmotionEntry::new(
        form.date.clone(),
        emotion,
        form.note.clone(),
    );

    save_or_update_today_entry("moods.csv", entry)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to save mood"))?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/list"))
        .finish())
}

fn save_entries(entries: &[EmotionEntry]) -> std::io::Result<()> {
    let mut file = std::fs::File::create("moods.csv")?;
    for entry in entries {
        writeln!(
            file,
            "{},{},{},{}",
            entry.date,
            entry.emotion,
            entry.color,
            entry.note.as_deref().unwrap_or("")
        )?;
    }
    Ok(())
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

async fn delete(form: web::Form<DeleteForm>) -> Result<HttpResponse> {
    let mut entries = load_entries("moods.csv").unwrap_or_default();
    entries.retain(|e| e.date != form.date); // remove the entry by date
    save_entries(&entries)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to save moods"))?;

    // redirect back to list page
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/list"))
        .finish())
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
            .app_data(web::Data::new(tera.clone()))
            .service(fs::Files::new("/static", "./static"))
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/submit", web::post().to(submit))
            .route("/list", web::get().to(list))
            .route("/blanket", web::get().to(blanket))
            .route("/delete", web::post().to(delete))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

