use tonic::{service, transport::Server, Request, Response, Status};
use k4::{
    k4_server::{K4Server, K4}, 
    Enter, 
    Entered
};

use std::{default, env, io};
use std::io::Write;

use crate::k4::k4_server;

pub mod k4 {
    tonic::include_proto!("k4");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let store = connect();
    print!("Redis: connection opened");

    let address = "[::1]:6379".parse().unwrap();
    let k_service = KService::default();

    println!("GRPC up");

    Server::builder().add_service(K4Server::new(k_service))
        .serve(address)
        .await?;
    Ok(())

}

#[derive(Debug, Default)]
pub struct KService {
}

#[tonic::async_trait]
impl K4 for KService {
    
    async fn to_enter(&self, request: Request<Enter>) -> Result<Response<Entered>, Status> {
        let r = request.into_inner();

        print!("entered");
        io::stdout().flush().unwrap();

        let response: Entered = Entered {
            access: "test".to_string(),
        };

        return Ok(Response::new(response))
    }
}

fn connect() -> redis::Connection {
    let redis_host_name = env::var("REDIS_HOSTNAME").unwrap_or("localhost".to_string());
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or("".to_string());
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "redis",
        Err(_) => "redis",
    };
    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);

    redis::Client::open(redis_conn_url)
        .expect("bad redi conn url")
        .get_connection()
        .expect("failed connect Redis")
}