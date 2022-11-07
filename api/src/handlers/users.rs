use actix_web::{web, Responder, Scope};
use app::UseCase;
use infra::repository_impls::RepositoryImpls;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SearchRequest {
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[derive(Serialize)]
struct SearchResponse {
    users: Vec<domain::User>,
}

async fn search(req: web::Query<SearchRequest>) -> impl Responder {
    let repo = RepositoryImpls::default();
    let use_case = UseCase::new(&repo);
    let users = use_case.search_users(
        req.firstname
            .clone()
            .map(domain::UserFirstName::new)
            .as_ref(),
        req.lastname.clone().map(domain::UserLastName::new).as_ref(),
        req.email.clone().map(domain::EmailAddress::new).as_ref(),
    );
    let res = SearchResponse { users };
    web::Json(res)
}

pub fn get_user() -> Scope {
    web::scope("/users").route("", web::get().to(search))
}
