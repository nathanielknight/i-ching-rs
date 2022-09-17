use askama::Template;
use axum::extract::Query;
use axum::response::{ErrorResponse, Html, Result};
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(get_input))
        .route("/throw", get(get_throw));
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 61849));
    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("error starting server");
}

#[derive(Deserialize)]
struct ThrowParams {
    prompt: String,
    asof: chrono::NaiveDate,
}

#[derive(Template)]
#[template(path = "throw.html")]

struct ThrowViewModel {
    prompt: String,
    asof: chrono::NaiveDate,
    throw: String,
}

#[derive(Deserialize, Template)]
#[template(path = "input.html")]
struct InputViewModel {}

type AppResponse = Result<Html<String>>;

pub(crate) async fn get_throw(Query(params): Query<ThrowParams>) -> AppResponse {
    let mut thrower = Thrower::new(&params);
    let throw = format_throws(thrower.throws());
    let viewmodel = ThrowViewModel {
        prompt: params.prompt,
        asof: params.asof,
        throw,
    };
    viewmodel
        .render()
        .map(|b| Html(b))
        .map_err(|_| ErrorResponse::from("Template render error"))
}

pub(crate) async fn get_input() -> Html<String> {
    let input = InputViewModel {};
    Html(input.render().expect("Failed to render static template!"))
}

fn seed(params: &ThrowParams) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&params.prompt);
    hasher.update(&format!("{}", params.asof));
    hasher.finalize().into()
}

struct Thrower {
    rng: rand::rngs::StdRng,
}

impl Thrower {
    fn new(params: &ThrowParams) -> Self {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::from_seed(seed(params));
        Thrower { rng }
    }

    fn coin(&mut self) -> u8 {
        use rand::Rng;
        if self.rng.gen_bool(0.5) {
            2
        } else {
            3
        }
    }

    fn throw(&mut self) -> u8 {
        self.coin() + self.coin() + self.coin()
    }

    fn throws(&mut self) -> [u8; 6] {
        let mut throws = [0; 6];
        for idx in 0..6 {
            throws[idx] = self.throw();
        }
        throws
    }
}

fn format_throw(t: u8) -> &'static str {
    match t {
        6 => "--- x --- 6",
        7 => "--------- 7",
        8 => "---   --- 8",
        9 => "----o---- 9",
        _ => panic!("Impossible throw value : {}", t),
    }
}

fn format_throws(ts: [u8; 6]) -> String {
    let lines: Vec<String> = ts.iter().map(|t| format_throw(*t).to_owned()).collect();
    lines.join("\n")
}

#[test]
fn test_throwing() {
    const ALLOWED: [u8; 4] = [6, 7, 8, 9];

    let params = ThrowParams {
        prompt: String::from("Once there was a way to get back home."),
        asof: chrono::Local::today().naive_local(),
    };
    let mut thrower = Thrower::new(&params);

    for _ in 0..1_000_000 {
        let v = thrower.throw();
        assert!(ALLOWED.contains(&v));
    }

    let throws = thrower.throws();
    assert!(throws.iter().all(|t| ALLOWED.contains(&t)));
}
