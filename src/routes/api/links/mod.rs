mod _delete;
pub(self) mod _get;
mod _patch;
mod _post;

use crate::AppState;
use axum::routing::MethodRouter;

pub fn method_routes() -> MethodRouter<AppState> {
    MethodRouter::new()
        .get(_get::handler)
        .post(_post::handler)
        .patch(_patch::handler)
        .delete(_delete::handler)
}
