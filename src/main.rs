use std::{error::Error, net::SocketAddr};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::ToSchema;
use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Mutex;
use rusqlite::{params, Result};
use r2d2::{Pool};
use r2d2_sqlite::SqliteConnectionManager;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse},
    routing::{get, post, delete, put},
    Json, Router,
};
use lazy_static::lazy_static;

#[derive(OpenApi)]
#[openapi(paths(get_all_products, post_product, delete_product, update_product), components(schemas(Product)))]
pub struct ApiDoc;

lazy_static! {
    static ref POLL_CONNECTION: Mutex<Pool<SqliteConnectionManager>> = {
        let manager = SqliteConnectionManager::file(":memory:");
        let pool = Pool::new(manager).unwrap();
        Mutex::new(pool)
    };
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    create_table();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let app = Router::new()
        .route("/products", get(get_all_products))
        .route("/products", post(post_product))
        .route("/products/:id", delete(delete_product))
        .route("/products/:id", put(update_product))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[utoipa::path(delete, path = "/products/{id}",
responses((status = 200, description = "Produto excluído", body = Product),
(status = NOT_FOUND, description = "Produto não encontrado")), params(("id" = Uuid, Path, description = "Identificador"),))]
async fn delete_product(Path(id): Path<Uuid>) -> Result<Json<Product>, impl IntoResponse> {
    match perform_delete_product(id).await {
        Ok(_) => Ok(Json(Product {
            id,
            name: "Deleted ".into(),
            price: 0.0,
        })),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete"),
        )),
    }
}

async fn perform_delete_product(id: Uuid) -> Result<(), String> {
    // Simulate an error for demonstration
    if id.is_nil() {
        Err("Cannot be deleted.".to_string())
    } else {
        let conn = POLL_CONNECTION.lock().unwrap().get().unwrap();
        conn.execute(
            "DELETE FROM product WHERE id = ?1",
            params![id.to_string()],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[utoipa::path(get, path = "/products",
responses((status = 200, description = "Lista todos os produtos")))]
async fn get_all_products() -> impl IntoResponse {
    match get_all_products_impl().await {
        Ok(products) => (StatusCode::OK, products),
        Err(_) => {
            let produtos_vazios: Vec<Product> = Vec::new();
            (StatusCode::INTERNAL_SERVER_ERROR, Json(produtos_vazios))
        },
    }
}

async fn get_all_products_impl() -> Result<Json<Vec<Product>>, rusqlite::Error> {
    let conn = POLL_CONNECTION.lock().unwrap().get().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, price FROM product")?;
    let product_iter = stmt.query_map([], |row| {
        Ok(Product {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            name: row.get(1)?,
            price: row.get(2)?,
        })
    })?;

    let mut products = Vec::new();
    for product in product_iter {
        products.push(product?);
    }
    Ok(Json(products))
}

#[utoipa::path(post, path = "/products",
responses((status = 200, description = "Adiciona um produto")))]
async fn post_product(Json(mut product): Json<Product>) -> Result<Json<Product>, impl IntoResponse>{
    match save_product(&mut product).await {
        Ok(_) => Ok({
            Json(product.clone())
        }),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete"),
        )),
    }
}

async fn save_product(product: &mut Product) -> Result<(), Product> {
    product.id = Uuid::new_v4();
    let conn = POLL_CONNECTION.lock().unwrap().get().unwrap();
    create_table();
    conn.execute(
        "INSERT INTO product (id, name, price) VALUES (?1, ?2, ?3);",
        params![product.id.to_string(), product.name, product.price],
    ).unwrap();
    Ok(())
}

#[utoipa::path(put, path = "/products/{id}",
responses((status = 200, description = "Atualiza um produto"),
(status = NOT_FOUND, description = "Produto não encontrado")), params(("id" = Uuid, Path, description = "Identificador"),))]
async fn update_product(Path(id): Path<Uuid>, Json(mut product): Json<Product>) -> Result<Json<Product>, impl IntoResponse>{
    match update_product_impl(id, &mut product).await {
        Ok(_) => Ok({
            Json(product.clone())
        }),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update"),
        )),
    }
}

async fn update_product_impl(id: Uuid, product: &mut Product) -> Result<(), String> {
    let conn = POLL_CONNECTION.lock().unwrap().get().unwrap();
    conn.execute(
        "UPDATE product SET name = ?1, price = ?2 WHERE id = ?3;",
        params![product.name, product.price, id.to_string()],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

fn create_table() {
    let conn = POLL_CONNECTION.lock().unwrap().get().unwrap();

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS product (
                  id TEXT PRIMARY KEY,
                  name TEXT NOT NULL,
                  price REAL NOT NULL
                  );",
        [],
    ).unwrap() {
        0 => println!(),
        _ => println!(),
    };
}

#[derive(Clone,Serialize,Deserialize,ToSchema)]
struct Product {
    #[schema(default = "00000000-0000-0000-0000-000000000000", example = "00000000-0000-0000-0000-000000000000")]
    id: Uuid,
    name: String,
    price: f32,
}
