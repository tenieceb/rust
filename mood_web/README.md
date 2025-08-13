# Overview

MoodStitch is a web-based mood journal designed to help users track their emotions and associate them with colors. The app not only allows daily mood entries but also provides a visual representation of mood trends over time, known as the "Mood Blanket."

To start the app locally, clone the repository, run:

```bash
cargo run
```

Your default browser should open to [http://127.0.0.1:8080](http://127.0.0.1:8080) to see the homepage.

The purpose of this project is to further my understanding of Rust as a full-stack language, integrating a Rust back-end with a web front-end using Actix Web and Tera templates. It also demonstrates handling form submissions, file I/O, and dynamic page rendering.

[Software Demo Video](http://youtube.link.goes.here)

# Web Pages

- **Homepage (**``**)**: Users can submit a new mood entry. The form supports selecting pre-defined emotions or entering a custom emotion.
- **Past Moods (**``**)**: Displays a table of all previous entries, sorted by newest first. Users can also delete entries from this page.
- **Mood Blanket (**``**)**: Displays a color grid representing each recorded mood, providing a visual snapshot of emotional trends over time.

# Development Environment

- **Tools**: Visual Studio Code, Cargo, Rust Analyzer
- **Programming Language**: Rust
- **Libraries/Crates**: Actix Web (web framework), Tera (templating engine), Serde (serialization), Chrono (date handling), Webbrowser (auto-open browser), Actix Files (static files handling)

# Useful Websites

- [Actix Web Official Documentation](https://actix.rs/docs/)
- [Tera Templates Documentation](https://keats.github.io/tera/docs/)
- [Serde Documentation](https://serde.rs/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)

# Future Work

- Implement user authentication to store multiple users’ mood journals.
- Enhance the Mood Blanket visualization with interactive features and improved styling.
- Add more robust error handling for file I/O and form submissions.
- Support exporting mood data as CSV or JSON for analysis.
- Implement mobile app

