// CRATES
use crate::utils::{prefs, Preferences};
use actix_web::{cookie::Cookie, web::Form, HttpRequest, HttpResponse};
use askama::Template;
use time::{Duration, OffsetDateTime};

// STRUCTS
#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
	prefs: Preferences,
}

#[derive(serde::Deserialize)]
pub struct SettingsForm {
	theme: Option<String>,
	front_page: Option<String>,
	layout: Option<String>,
	wide: Option<String>,
	comment_sort: Option<String>,
	show_nsfw: Option<String>,
}

// FUNCTIONS

// Retrieve cookies from request "Cookie" header
pub async fn get(req: HttpRequest) -> HttpResponse {
	let s = SettingsTemplate { prefs: prefs(req) }.render().unwrap();
	HttpResponse::Ok().content_type("text/html").body(s)
}

// Set cookies using response "Set-Cookie" header
pub async fn set(_req: HttpRequest, form: Form<SettingsForm>) -> HttpResponse {
	let mut res = HttpResponse::Found();

	let names = vec!["theme", "front_page", "layout", "wide", "comment_sort", "show_nsfw"];
	let values = vec![&form.theme, &form.front_page, &form.layout, &form.wide, &form.comment_sort, &form.show_nsfw];

	for (i, name) in names.iter().enumerate() {
		match values[i] {
			Some(value) => res.cookie(
				Cookie::build(name.to_owned(), value)
					.path("/")
					.http_only(true)
					.expires(OffsetDateTime::now_utc() + Duration::weeks(52))
					.finish(),
			),
			None => res.del_cookie(&Cookie::named(name.to_owned())),
		};
	}

	res
		.content_type("text/html")
		.set_header("Location", "/settings")
		.body(r#"Redirecting to <a href="/settings">settings</a>..."#)
}
