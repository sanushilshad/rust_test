mod notification;
mod product;
mod routes;
pub(crate) mod user;
mod utils;
use notification::notification_route;
use product::product_route;
pub use routes::*;
use user::*;

use utils::util_route;
