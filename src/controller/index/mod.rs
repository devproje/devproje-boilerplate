use axum::{body::Bytes, http::{self, HeaderMap, Request, StatusCode}, response::IntoResponse};
use include_dir::{include_dir, Dir};
use mime_guess::from_path;

static DIST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/public");

pub async fn spa_handler(req: Request<axum::body::Body>) -> impl IntoResponse {
	let path = req.uri().path().trim_start_matches('/');
	let file_path = if path.is_empty() || !path.contains(".") {
		"index.html".to_string()
	} else {
		path.to_string()
	};

	if let Some(file) = DIST.get_file(&file_path) {
		let body = Bytes::from_static(file.contents());
		let mime = from_path(&file_path).first_or_octet_stream();

		let mut headers = HeaderMap::new();
		headers.insert(
			http::header::CONTENT_TYPE,
			mime.as_ref().parse().unwrap()
		);

		return (StatusCode::OK, headers, body).into_response();
	}

	return StatusCode::NOT_FOUND.into_response();
}
