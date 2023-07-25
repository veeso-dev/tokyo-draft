use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App as ActixApp, HttpServer};

use crate::hook::RenderHook;
use crate::render::RenderService;

mod health_check;
mod render;

pub struct WebServer {
    server: Server,
}

pub struct WebserverData {
    render_hook: Option<RenderHook>,
    render_service: RenderService,
}

impl WebServer {
    /// Initialize web server
    pub async fn init(
        listener_address: &str,
        render_hook: Option<String>,
        templates_dir: &str,
    ) -> anyhow::Result<Self> {
        info!("webserver initialized");
        info!("web listener address: {listener_address}");

        let listener = TcpListener::bind(listener_address)?;
        let render_service = RenderService::try_from(templates_dir)?;

        let server = {
            HttpServer::new(move || {
                let render_hook = render_hook.clone().map(RenderHook::from);
                let render_service = render_service.clone();
                let web_data = Data::new(WebserverData {
                    render_hook,
                    render_service,
                });
                ActixApp::new()
                    .service(health_check::check_action)
                    .service(render::render_action)
                    .app_data(web_data)
                    .wrap(Logger::default())
            })
            .listen(listener)?
            .run()
        };

        info!("web server initialized");
        Ok(Self { server })
    }

    /// Run web server
    pub async fn run(self) -> anyhow::Result<()> {
        info!("running web server");
        self.server.await?;
        info!("web server stopped");
        Ok(())
    }
}
