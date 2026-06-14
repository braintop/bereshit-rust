use actix_web::web;

use crate::controllers::recipe_controller::{
    create_recipe, delete_recipe, get_all_recipes, get_recipe, update_recipe,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_recipes)
        .service(get_recipe)
        .service(create_recipe)
        .service(update_recipe)
        .service(delete_recipe);
}
