use sycamore_router::*;

mod index;
mod login;
mod logout;
mod page;
mod test;
mod topic;
mod user;

pub use index::*;
pub use login::*;
pub use logout::*;
pub use page::*;
pub use test::*;
pub use topic::*;
pub use user::*;

#[derive(Route)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/login")]
    Login,
    #[to("/logout")]
    Logout,
    #[to("/topic/<id>/<page>")]
    Topic { id: i64, page: usize },
    #[to("/test")]
    Test,
    #[not_found]
    NotFound,
}
