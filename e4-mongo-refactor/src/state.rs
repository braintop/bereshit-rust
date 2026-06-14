use mongodb::Collection;

use crate::models::recipe::Recipe;
use crate::models::todo::Todo;

/// Shared application state injected into every handler via `web::Data`.
pub struct AppState {
    pub todos_collection: Collection<Todo>,
    pub recipes_collection: Collection<Recipe>,
}
