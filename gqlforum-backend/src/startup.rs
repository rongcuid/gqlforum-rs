use std::{env, fs};
use async_graphql::{EmptySubscription, Schema};
use axum::{Extension, handler::Handler, http::HeaderValue, Router, routing::get};

use sqlx::{ConnectOptions, sqlite::SqliteConnectOptions, SqlitePool};

use futures::executor::block_on;
use std::str::FromStr;
use std::sync::Arc;
use perseus::engine::{engine_build, engine_export, EngineOperation};
use perseus::{Html, PerseusApp, PerseusAppBase, SsrNode, Template};
use perseus::i18n::TranslationsManager;
use perseus::plugins::PluginAction;
use perseus::server::{ServerOptions, ServerProps};
use perseus::stores::MutableStore;
use perseus_axum::get_router;
use tower::builder::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::log::LevelFilter;

use crate::configuration::get_configuration;
use crate::backend::graphql::{MutationRoot, QueryRoot};
use crate::backend::routes::{
    fallback::handler_404,
    graphql::{graphql_handler, graphql_playground},
};

use crate::telemetry::{init_telemetry, setup_telemetry};

#[derive(Clone)]
pub struct HmacSecret(pub String);

#[derive(Clone)]
pub struct SessionCookieName(pub String);

fn get_props<M: MutableStore, T: TranslationsManager>(
    app: PerseusAppBase<SsrNode, M, T>,
) -> ServerProps<M, T> {
    if !cfg!(debug_assertions) {
        let binary_loc = env::current_exe().unwrap();
        let binary_dir = binary_loc.parent().unwrap(); // It's a file, there's going to be a parent if we're working on anything close
                                                       // to sanity
        env::set_current_dir(binary_dir).unwrap();
    }

    let plugins = app.get_plugins();

    plugins
        .functional_actions
        .server_actions
        .before_serve
        .run((), plugins.get_plugin_data());

    let static_dir_path = app.get_static_dir();

    let app_root = app.get_root();
    let immutable_store = app.get_immutable_store();
    let index_view_str = app.get_index_view_str();
    // By the time this binary is being run, the app has already been built be the
    // CLI (hopefully!), so we can depend on access to the render config
    let index_view = block_on(PerseusAppBase::<SsrNode, M, T>::get_html_shell(
        index_view_str,
        &app_root,
        &immutable_store,
        &plugins,
    ));

    let opts = ServerOptions {
        // We don't support setting some attributes from `wasm-pack` through plugins/`PerseusApp`
        // because that would require CLI changes as well (a job for an alternative engine)
        html_shell: index_view,
        js_bundle: "dist/pkg/perseus_engine.js".to_string(),
        // Our crate has the same name, so this will be predictable
        wasm_bundle: "dist/pkg/perseus_engine_bg.wasm".to_string(),
        // This probably won't exist, but on the off chance that the user needs to support older
        // browsers, we'll provide it anyway
        wasm_js_bundle: "dist/pkg/perseus_engine_bg.wasm.js".to_string(),
        templates_map: app.get_atomic_templates_map(),
        locales: app.get_locales(),
        root_id: app_root,
        snippets: "dist/pkg/snippets".to_string(),
        error_pages: Arc::new(app.get_error_pages()),
        // This will be available directly at `/.perseus/static`
        static_dir: if fs::metadata(&static_dir_path).is_ok() {
            Some(static_dir_path)
        } else {
            None
        },
        static_aliases: app.get_static_aliases(),
    };

    ServerProps {
        opts,
        immutable_store,
        mutable_store: app.get_mutable_store(),
        global_state_creator: app.get_global_state_creator(),
        translations_manager: block_on(app.get_translations_manager()),
    }
}

// #[perseus::browser_main]
pub fn perseus_app<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
}

pub async fn run() -> i32 {
    init_telemetry();
    let configuration = get_configuration().expect("Failed to read configuration");
    let addr = format!("{}:{}", configuration.listen, configuration.port)
        .parse()
        .unwrap();

    let op = perseus::engine::get_op().expect("perseus engine operation");

    match op {
        EngineOperation::Build => {
            engine_build(perseus_app()).await.unwrap();
        }
        EngineOperation::Export => {
            engine_export(perseus_app()).await.unwrap();
        }
        EngineOperation::ExportErrorPage => {
            unimplemented!()
        }
        EngineOperation::Serve => {
            let props = get_props(perseus_app());
            let mut options = SqliteConnectOptions::from_str(&configuration.database.connection)
                .expect("Failed to create SqlitePoolOptions")
                .create_if_missing(true)
                .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
                .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental)
                .pragma("temp_store", "MEMORY");
            options.log_statements(LevelFilter::Trace);
            let pool = SqlitePool::connect_with(options)
                .await
                .expect("SQLite connection error");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .expect("Migration error");

            let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
                .extension(async_graphql::extensions::Tracing)
                // .extension(async_graphql::extensions::ApolloTracing)
                .data(HmacSecret(configuration.hmac_secret.clone()))
                .data(SessionCookieName(configuration.session_cookie_name.clone()))
                .data(pool.clone())
                .finish();

            // let props: ServerProps = Default::default();
            // build our application with a route
            let app = get_router(props).await
                .route("/graphql", get(graphql_playground).post(graphql_handler))
                .fallback(handler_404.into_service())
                .layer(
                    ServiceBuilder::new()
                        // .layer(
                        // CorsLayer::new()
                        // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap()),
                        // )
                        .layer(Extension(pool))
                        .layer(Extension(schema))
                        .layer(Extension(SessionCookieName(
                            configuration.session_cookie_name.clone(),
                        )))
                        .layer(Extension(HmacSecret(configuration.hmac_secret.clone()))),
                );

            let app = setup_telemetry(app);

            // run it

            tracing::debug!("listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        EngineOperation::Tinker => {}
    }
    0
}
