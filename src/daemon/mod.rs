mod response;

use response::{SignedUrlResponse, SignedUrlAttributes};
use crate::config_file::ConfigurationControl;
use crate::signed_url::SignedUrlRunner;
use crate::command_control;
use crate::command_control::CmdCtl;
use actix_web::{web, App, HttpServer, HttpRequest, Responder, HttpResponse};
use structopt::StructOpt;
use uuid::Uuid;


pub struct Daemeon { }

impl Daemeon {

    pub fn update_put_options(bucket: String, mut key: String, options: &mut CmdCtl) {
        if !options.no_edit_bucket {
            options.bucket = Some(bucket);
        } else {
            // If we are not using buckets that means bucket & key equal a key.
            let mut new_key = bucket;
            new_key.push_str("/");
            new_key.push_str(&key);
            key = new_key;
        }
        match &options.prefix {
            Some(prefix) => {
                let mut new_key = prefix.clone();
                new_key.push_str("/");
                if options.generate_key {
                    new_key.push_str(&Uuid::new_v4().to_simple().to_string());
                } else {
                    new_key.push_str(&key);
                }
                options.key = Some(new_key);
            },
            None => {
                let new_key = if options.generate_key { Uuid::new_v4().to_simple().to_string() } else { key };
                options.key = Some(new_key);
            },
        }
    }

    pub fn update_existing_object_options(bucket: String, mut key: String, options: &mut CmdCtl) {
        if !options.no_edit_bucket {
            options.bucket = Some(bucket);
        } else {
            // If we are not using buckets that means bucket & key equal a key.
            let mut new_key = bucket;
            new_key.push_str("/");
            new_key.push_str(&key);
            key = new_key;
        }
        match &options.prefix {
            Some(prefix) => {
                let mut new_key = prefix.clone();
                new_key.push_str("/");
                new_key.push_str(&key);
                options.key = Some(new_key);
            },
            None => {
                options.key = Some(key);
            },
        }
    }
    pub fn to_json(url: String, method: String, ttl: i64, origin: String) -> SignedUrlResponse {
        let mut json = SignedUrlResponse::new();
        let mut attributes = SignedUrlAttributes::new();

        attributes.set_url(url);
        attributes.set_method(method);
        attributes.set_ttl(ttl);
        attributes.set_engine("aws".to_string());
        attributes.set_request(origin);
        json.set_attributes(attributes);

        json
    }

    pub async fn put_key() -> impl Responder {
        let mut options = command_control::CmdCtl::from_args();

        options.method = "PUT".to_string();
        match &options.prefix {
            Some(prefix) => {
                let mut new_key = prefix.clone();
                new_key.push_str("/");
                new_key.push_str(&Uuid::new_v4().to_simple().to_string());
                options.key = Some(new_key);
            },
            None => {
                options.key = Some(Uuid::new_v4().to_simple().to_string());
            },
        }
        let signed_url = SignedUrlRunner::run(&options).await;
        let json = Daemeon::to_json(signed_url.unwrap(), "PUT".to_string(), 60, "/create".to_string());

        HttpResponse::Ok().json(json)
    }

    pub async fn put_key_conversion(req: HttpRequest) -> impl Responder {
        let (mut bucket, key): (String, String) = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();

        options.method = "PUT".to_string();
        Daemeon::update_put_options(bucket.clone(), key.clone(), &mut options);

        let signed_url = SignedUrlRunner::run(&options).await;
        bucket.push_str("/");
        bucket.push_str(&key);
        let json = Daemeon::to_json(signed_url.unwrap(), "PUT".to_string(), 60, bucket);

        HttpResponse::Ok().json(json)
    }

    pub async fn get_key_conversion(req: HttpRequest) -> impl Responder {
        let (mut bucket, key): (String, String) = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();

        options.method = "GET".to_string();
        Daemeon::update_existing_object_options(bucket.clone(), key.clone(), &mut options);

        let signed_url = SignedUrlRunner::run(&options).await;
        bucket.push_str("/");
        bucket.push_str(&key);
        let json = Daemeon::to_json(signed_url.unwrap(), "GET".to_string(), 60, bucket);
        HttpResponse::Ok().json(json)
    }

    pub async fn del_key_conversion(req: HttpRequest) -> impl Responder {
        let (mut bucket, key): (String, String) = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();

        options.method = "DELETE".to_string();
        Daemeon::update_existing_object_options(bucket.clone(), key.clone(), &mut options);

        let signed_url = SignedUrlRunner::run(&options).await;
        bucket.push_str("/");
        bucket.push_str(&key);
        let json = Daemeon::to_json(signed_url.unwrap(), "GET".to_string(), 60, bucket);
        HttpResponse::Ok().json(json)
    }

    pub async fn run_as_daemeon() -> std::io::Result<()> {
        let options = command_control::CmdCtl::from_args();
        let host = format!("{}:{}", options.host, options.port);
        ConfigurationControl::new().load();
        println!("Listening {:#?}", host);
        HttpServer::new(|| {
            App::new()
                .route("/create", web::get().to( Daemeon::put_key ))
                .route("/create/{bucket}/{key:.*}", web::get().to( Daemeon::put_key_conversion ))
                .route("/read/{bucket}/{key:.*}", web::get().to( Daemeon::get_key_conversion ))
                .route("/delete/{bucket}/{key:.*}", web::get().to( Daemeon::del_key_conversion ))
        })
        .bind(host)?
        .run()
        .await
    }

}
