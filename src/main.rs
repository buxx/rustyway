use axum::{
    Form, Router,
    response::Html,
    routing::{get, post},
};
use cmd_lib::run_cmd;
use serde::Deserialize;
use tracing_unwrap::ResultExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(root)).route("/", post(do_it));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6479").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> Html<String> {
    let title = "🪁 Restart server ?";

    Html(format!(
        "
        <h1>{}</h1>
        <form action=\"/\" method=\"post\">
            <input type=\"text\" name=\"password\" />
            <input type=\"submit\" value=\"Yes\" />
        </form>
    ",
        title
    ))
}

#[derive(Deserialize)]
struct DoIt {
    password: String,
}

async fn do_it(Form(payload): Form<DoIt>) -> Html<&'static str> {
    if &payload.password != "zinzins" {
        return Html("<h1>🤔</h1>");
    }

    std::thread::spawn(|| {
        run_cmd! (
            cd /
            /usr/bin/docker compose -f docker-compose-enshrouded.yml stop
            sleep 5
            /usr/bin/docker compose -f docker-compose-enshrouded.yml start
        )
        .unwrap_or_log();
    });

    Html("✨ On the way !")
}
