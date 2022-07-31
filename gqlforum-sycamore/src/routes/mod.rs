use sycamore_router::*;

mod index;
mod login;
mod logout;
mod topic;
mod user;
mod test;

pub use index::*;
pub use login::*;
pub use logout::*;
pub use topic::*;
pub use user::*;
pub use test::*;

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
    #[to("/user/<id>")]
    User { id: i64 },
    #[to("/test")]
    Test,
    #[not_found]
    NotFound,
}
