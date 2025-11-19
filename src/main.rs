use axum::{
    Router,
    response::Html,
    routing::{get, post},
};
use cmd_lib::run_cmd;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(root)).route("/", post(do_it));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6479").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> Html<String> {
    let title = "Restart server ?";

    Html(format!(
        "
        <h1>{}</h1>
        <form action=\"/\">
            <input type=\"submit\" value=\"Yes\" />
        </form>
    ",
        title
    ))
}

async fn do_it() -> Html<&'static str> {
    std::thread::spawn(|| {
        let _ = run_cmd! (
            cd /
            /usr/bin/docker compose -f docker-compose-enshrouded.yml stop
            sleep 5
            /usr/bin/docker compose -f docker-compose-enshrouded.yml start
        );
    });

    Html("On the way !")
}
