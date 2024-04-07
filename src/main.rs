use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    routing::{get, post, Router},
    Form, Json,
};
use axum_extra::TypedHeader;
use headers::UserAgent;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/user/:id", get(user_info))
        .route("/repo/:user/:repo", get(repo_info))
        .route("/repo_struct/:user_name/:repo_name", get(repo_info_struct))
        .route("/subject", get(subject))
        .route("/subject_opt", get(subject_opt))
        .route("/subject_opt_done", get(subject_opt_done))
        .route("/all_query", get(all_query))
        .route("/create_user", post(create_user))
        .route("/create_user_json", post(create_user_json))
        .route("/headers", get(get_all_headers))
        .route("/get_user_agent", get(get_user_agent_typed));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn user_info(id: Path<i32>) -> String {
    format!("User info for {}", id.0)
}

async fn repo_info(Path((user_name, repo_name)): Path<(String, String)>) -> String {
    format!(
        "Repository: user name: {} and repository name: {}",
        user_name, repo_name
    )
}

#[derive(Deserialize)]
pub struct RepoInfo {
    pub user_name: String,
    pub repo_name: String,
}

async fn repo_info_struct(Path(info): Path<RepoInfo>) -> String {
    format!(
        "Repository: user name: {} and repository name: {}",
        info.user_name, info.repo_name
    )
}

#[derive(Deserialize)]
pub struct SubjectArgs {
    pub page: i32,
    pub keyword: String,
}

async fn subject(Query(args): Query<SubjectArgs>) -> String {
    format!("Page {}, keyword: {} of subjects", args.page, args.keyword)
}

async fn subject_opt(args: Option<Query<SubjectArgs>>) -> String {
    if let Some(args) = args {
        let args = args.0;

        return format!("Page {}, keyword: {} of subjects", args.page, args.keyword);
    }

    "Page 0, no keyword of subjects".to_string()
}

#[derive(Deserialize)]
pub struct SubjectArgsOpt {
    pub page: Option<i32>,
    pub keyword: Option<String>,
}

async fn subject_opt_done(Query(args): Query<SubjectArgsOpt>) -> String {
    let page = args.page.unwrap_or(0);
    let keyword = args.keyword.unwrap_or("".to_string());

    format!("Page {}, keyword: {} of subjects", page, keyword)
}

async fn all_query(Query(args): Query<HashMap<String, String>>) -> String {
    format!("{:?}", args)
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub level: u8,
}

async fn create_user(Form(frm): Form<CreateUser>) -> String {
    format!(
        "Created user: {}, email: {}, level: {}",
        frm.username, frm.email, frm.level
    )
}

async fn create_user_json(Json(frm): Json<CreateUser>) -> String {
    format!(
        "Created user: {}, email: {}, level: {}",
        frm.username, frm.email, frm.level
    )
}

async fn get_all_headers(headers: HeaderMap) -> String {
    format!("{:?}", headers)
}

async fn get_user_agent_typed(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}
