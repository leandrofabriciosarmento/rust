use std::{error::Error, net::SocketAddr};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::ToSchema;
use serde::Serialize;
use serde::Deserialize;
use uuid::Uuid;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse},
    routing::{get, post, delete},
    Json, Router,
};

#[derive(OpenApi)]
#[openapi(paths(get_all_products, post_product, delete_product), components(schemas(Product)))]
pub struct ApiDoc;

// Array de Products estatica viisvel para todos os metodos
static mut PRODUCTS: Vec<Product> = Vec::new();


//noinspection ALL
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let app = Router::new()
        .route("/products", get(get_all_products))
        .route("/products", post(post_product))
        .route("/products/:id", delete(delete_product))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[utoipa::path(delete, path = "/products/{id}",
responses((status = 200, description = "Pet found successfully", body = Pet),
(status = NOT_FOUND, description = "Pet was not found")),
params(("id" = Uuid, Path, description = "Identificador"),))]
async fn delete_product(Path(id): Path<Uuid>) -> Result<Json<Product>, impl IntoResponse> {
    match perform_delete_product(id).await {
        Ok(_) => Ok(Json(Product {
            id: id,
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
        for (i, product) in unsafe { PRODUCTS.iter() }.enumerate() {
            if product.id == id {
                unsafe { PRODUCTS.remove(i); }
                break;
            }
        }
        Ok(())
    }
}

#[utoipa::path(get, path = "/products",
responses((status = 200, description = "Lista todos os produtos")))]
async fn get_all_products() -> Json<Vec<Product>>{
    Json(unsafe { PRODUCTS.clone() })
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
    unsafe { PRODUCTS.push(product.to_owned()); }
    Ok(())
}

#[derive(Clone,Serialize,Deserialize,ToSchema)]
struct Product {
    #[schema(default = "00000000-0000-0000-0000-000000000000", example = "00000000-0000-0000-0000-000000000000")]
    id: Uuid,
    name: String,
    price: f32,
}