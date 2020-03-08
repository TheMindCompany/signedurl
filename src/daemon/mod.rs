use crate::config_file::ConfigurationControl;
use crate::signed_url::SignedUrlRunner;
use crate::command_control;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use structopt::StructOpt;

pub struct Daemeon { }

impl Daemeon {

    pub async fn dameon_mode_conversion() -> impl Responder {
        let options = command_control::CmdCtl::from_args();
        let signed_url = SignedUrlRunner::run(&options).await;
        HttpResponse::Ok().body(signed_url.unwrap())
    }

    pub async fn run_as_daemeon() -> std::io::Result<()> {
        let options = command_control::CmdCtl::from_args();
        let host = format!("127.0.0.1:{:#?}", options.port);
        ConfigurationControl::new().load();

        println!("Listening {:#?}", host);
        HttpServer::new(|| {
            App::new()
                .route("/", web::get().to( Daemeon::dameon_mode_conversion ))
        })
        .bind(host)?
        .run()
        .await
    }

}
