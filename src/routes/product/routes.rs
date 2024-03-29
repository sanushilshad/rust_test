use super::views::product_search;
use actix_web::web;
pub fn product_route(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::resource("/inventory/fetch").route(web::post().to(fetch_inventory)));
    cfg.service(web::resource("/search").route(web::post().to(product_search)));

    // cfg.route("/customer/database", web::post().to(get_customer_dbs_api))
}
