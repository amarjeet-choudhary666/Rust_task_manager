use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::{doc, Bson}, Client};
use mongodb::bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use chrono::Utc;
use crate::models::task_model::{NewTask, Task, TaskResponse, UpdateTask, TaskStatus};

pub async fn create_task(
    task: web::Json<NewTask>,
    client: web::Data<Client>,
) -> impl Responder {
    if task.title.trim().is_empty() {
        return HttpResponse::BadRequest().body("Title is required");
    }

    let database = client.database("rust_backend");
    let collection = database.collection::<Task>("tasks");

    let now = Utc::now();
    let new_task = Task {
        id: None,
        title: task.title.clone(),
        description: task.description.clone(),
        status: task.status.as_ref().cloned().unwrap_or(TaskStatus::Pending),
        user_id: task.user_id.clone(),
        created_at: now,
        updated_at: now,
    };

    match collection.insert_one(new_task).await {
        Ok(insert_result) => {
            if let Some(oid) = insert_result.inserted_id.as_object_id() {
                let response = TaskResponse {
                    id: Some(oid.to_hex()),
                    title: task.title.clone(),
                    description: task.description.clone(),
                    status: task.status.as_ref().cloned().unwrap_or(TaskStatus::Pending),
                    user_id: task.user_id.clone(),
                    created_at: now,
                    updated_at: now,
                };
                HttpResponse::Ok().json(response)
            } else {
                HttpResponse::InternalServerError().body("Failed to get inserted ID")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error inserting task: {}", err)),
    }
}

pub async fn get_tasks(
    user_id: web::Path<String>,
    client: web::Data<Client>,
) -> impl Responder {
    let database = client.database("rust_backend");
    let collection = database.collection::<Task>("tasks");

    let cursor = match collection.find(doc! { "user_id": user_id.as_str() }).await {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    let tasks: Vec<TaskResponse> = match cursor.try_collect::<Vec<_>>().await {
        Ok(docs) => docs.into_iter().map(|task| TaskResponse {
            id: task.id,
            title: task.title,
            description: task.description,
            status: task.status,
            user_id: task.user_id,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }).collect(),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error reading tasks: {}", e)),
    };

    HttpResponse::Ok().json(tasks)
}

pub async fn get_task(
    path: web::Path<(String, String)>,
    client: web::Data<Client>,
) -> impl Responder {
    let (user_id, task_id) = path.into_inner();

    let database = client.database("rust_backend");
    let collection = database.collection::<Task>("tasks");

    let oid = match ObjectId::parse_str(&task_id) {
        Ok(o) => o,
        Err(_) => return HttpResponse::BadRequest().body("Invalid task ID"),
    };

    match collection.find_one(doc! { "_id": Bson::ObjectId(oid), "user_id": user_id }).await {
        Ok(Some(task)) => {
            let response = TaskResponse {
                id: task.id,
                title: task.title,
                description: task.description,
                status: task.status,
                user_id: task.user_id,
                created_at: task.created_at,
                updated_at: task.updated_at,
            };
            HttpResponse::Ok().json(response)
        }
        Ok(None) => HttpResponse::NotFound().body("Task not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
    }
}

pub async fn update_task(
    path: web::Path<(String, String)>,
    update: web::Json<UpdateTask>,
    client: web::Data<Client>,
) -> impl Responder {
    let (user_id, task_id) = path.into_inner();

    let database = client.database("rust_backend");
    let collection = database.collection::<Task>("tasks");

    let oid = match ObjectId::parse_str(&task_id) {
        Ok(o) => o,
        Err(_) => return HttpResponse::BadRequest().body("Invalid task ID"),
    };

    let mut update_doc = doc! { "updated_at": Utc::now().to_rfc3339() };
    if let Some(title) = &update.title {
        update_doc.insert("title", title);
    }
    if let Some(description) = &update.description {
        update_doc.insert("description", description);
    }
    if let Some(status) = &update.status {
        update_doc.insert("status", match status {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "InProgress",
            TaskStatus::Completed => "Completed",
        });
    }

    match collection.update_one(
        doc! { "_id": Bson::ObjectId(oid), "user_id": user_id },
        doc! { "$set": update_doc },
    ).await {
        Ok(update_result) => {
            if update_result.modified_count > 0 {
                HttpResponse::Ok().body("Task updated successfully")
            } else {
                HttpResponse::NotFound().body("Task not found")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error updating task: {}", err)),
    }
}

pub async fn delete_task(
    path: web::Path<(String, String)>,
    client: web::Data<Client>,
) -> impl Responder {
    let (user_id, task_id) = path.into_inner();

    let database = client.database("rust_backend");
    let collection = database.collection::<Task>("tasks");

    let oid = match ObjectId::parse_str(&task_id) {
        Ok(o) => o,
        Err(_) => return HttpResponse::BadRequest().body("Invalid task ID"),
    };

    match collection.delete_one(doc! { "_id": Bson::ObjectId(oid), "user_id": user_id }).await {
        Ok(delete_result) => {
            if delete_result.deleted_count > 0 {
                HttpResponse::Ok().body("Task deleted successfully")
            } else {
                HttpResponse::NotFound().body("Task not found")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error deleting task: {}", err)),
    }
}