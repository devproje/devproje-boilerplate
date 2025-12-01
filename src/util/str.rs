pub fn unwrap_string(v: String) -> &'static str {
	return Box::leak(v.into_boxed_str());
}
