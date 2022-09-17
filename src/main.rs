use askama::Template;
use axum::extract::Query;
use axum::response::{ErrorResponse, Html, Result};
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(get_input))
        .route("/throw", get(get_throw))
        .route("/about", get(get_about));
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

type AppResponse = Result<Html<String>>;

pub(crate) async fn get_throw(Query(params): Query<ThrowParams>) -> AppResponse {
    let mut thrower = Thrower::new(&params);
    let throw = format_hexagram(thrower.hexagram());
    let viewmodel = ThrowViewModel {
        prompt: params.prompt,
        asof: params.asof,
        throw,
    };
    viewmodel
        .render()
        .map(Html)
        .map_err(|_| ErrorResponse::from("Template render error"))
}

#[derive(Template)]
#[template(path = "input.html")]
struct InputViewModel {}

pub(crate) async fn get_input() -> Html<String> {
    let input = InputViewModel {};
    Html(input.render().expect("Failed to render input template!"))
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutViewModel {}

async fn get_about() -> Html<String> {
    let about = AboutViewModel {};
    Html(about.render().expect("Failed to render about page"))
}

fn seed(params: &ThrowParams) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&params.prompt);
    hasher.update(&format!("{}", params.asof));
    hasher.finalize().into()
}

struct Thrower {
    rng: rand_chacha::ChaCha12Rng,
}

impl Thrower {
    fn new(params: &ThrowParams) -> Self {
        use rand::SeedableRng;
        let rng = rand_chacha::ChaCha12Rng::from_seed(seed(params));
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

    fn throw(&mut self) -> Line {
        use Line::*;
        match self.coin() + self.coin() + self.coin() {
            6 => ChangingYin,
            7 => StaticYang,
            8 => StaticYin,
            9 => ChangingYang,
            t => panic!("Impossible throw value: {}", t),
        }
    }

    fn hexagram(&mut self) -> Hexagram {
        let mut throws = [Line::StaticYang; 6];
        for idx in 0..6 {
            throws[idx] = self.throw();
        }
        throws
    }
}

/// Yin -> broken line
/// Yang -> solid line
/// changing yin+x, yang+0
#[derive(Clone, Copy)]
enum Line {
    ChangingYin, // broken
    ChangingYang,
    StaticYin,
    StaticYang,
}

impl From<Line> for String {
    fn from(line: Line) -> String {
        use Line::*;
        match line {
            ChangingYin => "--- x --- 6",
            StaticYang => "--------- 7",
            StaticYin => "---   --- 8",
            ChangingYang => "----o---- 9",
        }
        .to_owned()
    }
}

type Hexagram = [Line; 6];

fn format_hexagram(ts: Hexagram) -> String {
    let lines: Vec<String> = ts.iter().map(|t| (*t).into()).collect();
    lines.join("\n")
}

#[test]
fn test_throwing() {
    // Try lots of throws to give us some confidence that the random logic
    // always falls within the expected bounds.

    let params = ThrowParams {
        prompt: String::from("Once there was a way to get back home."),
        asof: chrono::Local::today().naive_local(),
    };
    let mut thrower = Thrower::new(&params);

    for _ in 0..1_000_000 {
        thrower.throw();
    }

    thrower.hexagram();
}

#[test]
fn test_throw_stability() {}
