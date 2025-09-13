use std::sync::{Mutex, MutexGuard};

use crate::{
    AppState,
    model::{Item, JsonItem},
};
use actix_web::{HttpResponse, web};
use serde_json::json;
use uuid::Uuid;

pub async fn create_item(
    item: web::Json<JsonItem>,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    println!("Printing shit");
    let mut state = state.lock().unwrap();
    let item = item.into_inner();
    if state
        .items
        .iter()
        .any(|v| v.name.to_lowercase() == item.name.to_lowercase())
    {
        let n = format!("{} ismli user mavjud", item.name);
        return HttpResponse::BadRequest().json(json!(n));
    }
    let i = Item {
        id: Uuid::new_v4(),
        name: item.name,
        description: item.description,
    };
    state.items.push(i);
    HttpResponse::Ok().json(&state.items)
}

pub async fn get_items(items: web::Data<Mutex<AppState>>) -> HttpResponse {
    let state = items.lock().unwrap();
    HttpResponse::Ok().json(&state.items)
}

pub async fn get_item(id: web::Path<Uuid>, state: web::Data<Mutex<AppState>>) -> HttpResponse {
    let state = state.lock().unwrap();
    match state.items.iter().find(|i| i.id == *id) {
        Some(i) => HttpResponse::Ok().json(i),
        None => HttpResponse::NotFound().json(serde_json::json!({"error": "user not found"})),
    }
}

pub async fn update_item(
    id: web::Path<Uuid>,
    item: web::Json<JsonItem>,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    print!("UPDATED");
    let state = state.lock().unwrap();
    match state.items.iter().find(|i| i.id == *id) {
        Some(t) => {
            let itemm = Item {
                id: *id,
                name: item.name.to_string(),
                description: item.description.to_string(),
            };
            let mut s = delet_user(id, state).await;
            let c = itemm.clone();
            s.items.push(itemm);
            HttpResponse::Ok().json(c)
        }
        None => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "user con not be updated"}))
        }
    }
}

pub async fn delet_user(
    id: web::Path<Uuid>,
    mut s: MutexGuard<'_, AppState>,
) -> MutexGuard<'_, AppState> {
    if let Some(pos) = s.items.iter().position(|item| item.id == *id) {
        s.items.remove(pos);
    }
    println!("{} removed", id);
    s
}

pub async fn delet_user_api(id: web::Path<Uuid>, s: web::Data<Mutex<AppState>>) -> HttpResponse {
    let mut s = s.lock().unwrap();
    if let Some(pos) = s.items.iter().position(|item| item.id == *id) {
        s.items.remove(pos);
        return HttpResponse::Ok().json("success");
    } else {
        return HttpResponse::BadRequest().json(serde_json::json!("user not found to delete"));
    }
}
