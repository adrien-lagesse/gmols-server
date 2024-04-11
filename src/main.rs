use axum::extract::Path;
use axum::response::Json;
use axum::routing::get;
use axum::Router;
use public_search;
use tokio;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search/:cid", get(search_pubchem_compounds));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_pubchem_compounds(Path(cid): Path<u32>) -> Json<public_search::pubchem::Compound> {
    let client = public_search::pubchem::Client::new();
    let compound = client.get_compound_by_cid(cid).await.unwrap();
    Json(compound)
}
