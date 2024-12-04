use core::panic;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use graph::{Connection, MovieGraph};
use serde::Deserialize;

mod graph;

#[derive(Deserialize)]
struct BFSRequest {
    from: String,
    to: String,
}

#[derive(Clone)]
struct AppState {
    graph: Arc<Mutex<MovieGraph>>,
}

async fn get_bfs(bfs_req: Query<BFSRequest>, State(state): State<AppState>) -> &'static str {
    let req: BFSRequest = bfs_req.0;

    if req.from.is_empty() {
        return "KYS";
    }

    if req.to.is_empty() {
        return "KYS EVEN MORE";
    }

    let graph = state.graph.lock().unwrap();

    let huzz = graph.bfs_traversal(req.from, req.to);

    if let Some(chuzz) = huzz.linkage {
        for bruzz in chuzz {
            println!(
                "From {} to {} through {:?}",
                bruzz.from, bruzz.to, bruzz.actor
            );
        }
    } else {
        println!(
            "No connection found! But technically you didn't win buddy, get on over 6 connections"
        );
    }

    "Check the terminal\n"
}

#[tokio::main]
async fn main() {
    let state = AppState {
        graph: Arc::new(Mutex::new(MovieGraph::new())),
    };

    match state
        .graph
        .lock()
        .unwrap()
        .parse_csv(String::from("output.csv"))
    {
        Ok(_) => println!("Parsed successfully, begin server init"),
        Err(_) => {
            println!("Did not successfully init, crashing everything and burning");
            panic!("Buh bye!");
        }
    }

    let app: Router<()> = Router::new().route("/bfs", get(get_bfs)).with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    if let Ok(addr) = listener.local_addr() {
        println!("Server running on http://{}", addr);
    } else {
        panic!("No address binded!");
    }
    axum::serve(listener, app).await.unwrap();
}
