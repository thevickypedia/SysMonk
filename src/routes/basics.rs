
use actix_web::HttpResponse;


/// Handles the health endpoint, returning a JSON response indicating the server is healthy.
///
/// # Returns
///
/// Returns an `HttpResponse` with a status of 200 (OK), content type "application/json",
/// and a JSON body containing the string "Healthy".
#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json("Healthy")
}
