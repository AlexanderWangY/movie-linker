use core::panic;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use graph::{Connection, MovieGraph};
use serde::{Deserialize, Serialize};

mod graph;

#[derive(Deserialize)]
struct SearchRequest {
    from: String,
    to: String,
}

#[derive(Clone)]
struct AppState {
    graph: Arc<Mutex<MovieGraph>>,
}

#[derive(Serialize)]
struct ConnectionResponse {
    path: Vec<Connection>,
    time: u128,
    traversal_count: u32,
}

#[derive(Serialize)]
struct TimingResponse {
    found: bool,
    time: u128,
    traversal_count: u32,
}

#[derive(Serialize)]
struct NeighborResponse {
    path: Option<Connection>,
}

async fn get_bfs_connections(
    bfs_req: Query<SearchRequest>,
    State(state): State<AppState>,
) -> Json<ConnectionResponse> {
    let req: SearchRequest = bfs_req.0;

    let graph = state.graph.lock().unwrap();

    let huzz = graph.bfs_traversal(req.from, req.to);

    if let Some(chuzz) = huzz.linkage {
        Json(ConnectionResponse {
            path: chuzz,
            time: huzz.time_elapsed,
            traversal_count: huzz.traverse_count,
        })
    } else {
        Json(ConnectionResponse {
            path: vec![],
            time: huzz.time_elapsed,
            traversal_count: huzz.traverse_count,
        })
    }
}

async fn get_dfs_connections(
    dfs_req: Query<SearchRequest>,
    State(state): State<AppState>,
) -> Json<ConnectionResponse> {
    let req: SearchRequest = dfs_req.0;

    let graph = state.graph.lock().unwrap();

    let huzz = graph.dfs_traversal(req.from, req.to);

    if let Some(chuzz) = huzz.linkage {
        Json(ConnectionResponse {
            path: chuzz,
            time: huzz.time_elapsed,
            traversal_count: huzz.traverse_count,
        })
    } else {
        Json(ConnectionResponse {
            path: vec![],
            time: huzz.time_elapsed,
            traversal_count: huzz.traverse_count,
        })
    }
}

async fn get_dfs(
    dfs_req: Query<SearchRequest>,
    State(state): State<AppState>,
) -> Json<TimingResponse> {
    let req: SearchRequest = dfs_req.0;

    let graph = state.graph.lock().unwrap();

    let huzz = graph.newdfs(req.from, req.to);

    if huzz.linkage.is_some() {
        Json(TimingResponse {
            found: true,
            time: huzz.time_elapsed,
            traversal_count: huzz.traverse_count,
        })
    } else {
        Json(TimingResponse {
            found: false,
            time: huzz.time_elapsed,
            traversal_count: huzz.traverse_count,
        })
    }
}

async fn get_bfs(
    bfs_req: Query<SearchRequest>,
    State(state): State<AppState>,
) -> Json<TimingResponse> {
    let req: SearchRequest = bfs_req.0;

    let graph = state.graph.lock().unwrap();

    let huzz = graph.newbfs(req.from, req.to);

    if huzz.linkage.is_some() {
        Json(TimingResponse {
            found: true,
            time: huzz.time_elapsed,
            traversal_count: huzz.traverse_count,
        })
    } else {
        Json(TimingResponse {
            found: false,
            time: huzz.time_elapsed,
            traversal_count: huzz.traverse_count,
        })
    }
}

async fn connected(
    bfs_req: Query<SearchRequest>,
    State(state): State<AppState>,
) -> Json<NeighborResponse> {
    let req: SearchRequest = bfs_req.0;

    let graph = state.graph.lock().unwrap();

    let hawk = graph.is_connected(req.from, req.to);

    Json(NeighborResponse { path: hawk })
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

    let app: Router<()> = Router::new()
        .route("/bfs_connected", get(get_bfs_connections))
        .route("/bfs", get(get_bfs))
        .route("/dfs_connected", get(get_dfs_connections))
        .route("/dfs", get(get_dfs))
        .route("/connected", get(connected))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    if let Ok(addr) = listener.local_addr() {
        println!("Server running on http://{}", addr);
    } else {
        panic!("No address binded!");
    }
    axum::serve(listener, app).await.unwrap();
}
