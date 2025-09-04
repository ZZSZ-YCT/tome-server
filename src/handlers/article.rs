use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use json_patch::{Patch, PatchOperation};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde_json::Value;

use crate::entities::article::{ActiveModel, Model};
use crate::entities::prelude::Articles;
use crate::models::article::ArticleNoId;
use crate::security::api_key::AuthenticatedUser;

#[get("/article")]
pub async fn list_articles(db: web::Data<DatabaseConnection>) -> impl Responder {
    match Articles::find().all(db.get_ref()).await {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/article")]
pub async fn create_article(
    db: web::Data<DatabaseConnection>,
    article: web::Json<ArticleNoId>,
    _user: AuthenticatedUser,
) -> impl Responder {
    let model = ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        time: Set(article.time.clone()),
        contributor: Set(article.contributor.clone()),
        line: Set(article.line.clone()),
        unsure: Set(article.unsure),
        sensitive: Set(article.sensitive),
        attributes: Set(article.attributes.clone()),
    };

    match model.insert(db.get_ref()).await {
        Ok(inserted) => HttpResponse::Created().json(inserted),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/article/{id}")]
pub async fn get_article(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    match Articles::find_by_id(id).one(db.get_ref()).await {
        Ok(Some(article)) => HttpResponse::Ok().json(article),
        Ok(None) => HttpResponse::NotFound().body("Article not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/article/{id}")]
pub async fn put_article(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
    article: web::Json<ArticleNoId>,
    _user: AuthenticatedUser,
) -> impl Responder {
    let id = path.into_inner();

    let active_model = match Articles::find_by_id(id).one(db.get_ref()).await {
        Ok(Some(_)) => ActiveModel {
            id: Set(id.to_string()),
            time: Set(article.time.clone()),
            contributor: Set(article.contributor.clone()),
            line: Set(article.line.clone()),
            unsure: Set(article.unsure),
            sensitive: Set(article.sensitive),
            attributes: Set(article.attributes.clone()),
        },
        Ok(None) => return HttpResponse::NotFound().body("Article not found"),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    match active_model.update(db.get_ref()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

const ALLOWED_PATHS: &[&str] = &[
    "/time",
    "/contributor",
    "/line",
    "/unsure",
    "/sensitive",
    "/attributes",
];

fn validate_patch(patch: &Vec<PatchOperation>) -> Result<(), String> {
    for op in patch {
        let path = op.path();
        if ALLOWED_PATHS.contains(&path.as_str()) {
            continue;
        }
        if path.to_string().starts_with("/attributes/") {
            continue;
        }

        return Err(format!("Invalid patch path: {}", path));
    }
    Ok(())
}

#[patch("/article/{id}")]
pub async fn patch_article(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
    patch_json: web::Json<Value>,
    _user: AuthenticatedUser,
) -> impl Responder {
    let id = path.into_inner();

    let patch: Patch = match serde_json::from_value(patch_json.into_inner()) {
        Ok(p) => p,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid JSON Patch: {}", err)),
    };

    let Patch(ref operations) = patch;

    if let Err(err) = validate_patch(operations) {
        return HttpResponse::BadRequest().body(err);
    }

    let mut article_model: Model = match Articles::find_by_id(id).one(db.get_ref()).await {
        Ok(Some(article)) => article,
        Ok(None) => return HttpResponse::NotFound().body("Article not found"),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let mut article_value = serde_json::to_value(&article_model).unwrap();

    if article_value.get("attributes").is_none() || article_value["attributes"].is_null() {
        article_value["attributes"] = Value::Object(serde_json::Map::new());
    }

    if let Err(err) = json_patch::patch(&mut article_value, &patch) {
        return HttpResponse::BadRequest().body(format!("Failed to apply patch: {}", err));
    }

    article_model = match serde_json::from_value(article_value) {
        Ok(m) => m,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let active_model: ActiveModel = article_model.to_active_model();
    match active_model.update(db.get_ref()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/article/{id}")]
pub async fn delete_article(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
    _user: AuthenticatedUser,
) -> impl Responder {
    let id = path.into_inner();
    match Articles::delete_by_id(id).exec(db.get_ref()).await {
        Ok(res) => {
            if res.rows_affected == 0 {
                HttpResponse::NotFound().body("Article not found")
            } else {
                HttpResponse::NoContent().finish()
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
