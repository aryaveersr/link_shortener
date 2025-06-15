pub(self) mod _get;
mod _post;

use crate::AppState;
use axum::routing::{MethodRouter, get};

pub fn method_routes() -> MethodRouter<AppState> {
    get(_get::handler).post(_post::handler)
}
