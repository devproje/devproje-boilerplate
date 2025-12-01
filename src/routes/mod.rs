use axum::{Router};

pub async fn index() -> &'static str {
	return "Hello, World!";
}

pub fn api() -> Router {
	return Router::new();
}
