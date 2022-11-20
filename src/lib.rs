pub mod server {
    tonic::include_proto!("server");
}
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use server::{server_service_server::ServerService, InstallServerReply, StartServerReply};
use tokio::process::{Child, Command};
use tonic::{Response, Status};

pub struct Service {
    // install_bus: Sender<server::Server>,
    // run_bus: Sender<server::Server>,
    service_map: Arc<Mutex<HashMap<i32, Child>>>,
    steam_cmd: String,
    base_dir: String,
}

impl Service {
    pub fn new(
        // install_bus: Sender<server::Server>,
        // run_bus: Sender<server::Server>,
        steam_cmd: &str,
        base_dir: &str,
    ) -> Self {
        Service {
            // install_bus,
            // run_bus,
            service_map: Arc::new(Mutex::new(HashMap::new())),
            steam_cmd: steam_cmd.into(),
            base_dir: base_dir.into(),
        }
    }
}

#[tonic::async_trait]
impl ServerService for Service {
    async fn install_server(
        &self,
        request: tonic::Request<server::InstallServerRequest>,
    ) -> Result<tonic::Response<server::InstallServerReply>, tonic::Status> {
        println!("In install");
        // let tx = self.install_bus.clone();
        // if let Some(svr) = request.into_inner().server {
        //     tx.send(svr)
        //         .await
        //         .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{}", e)))?;
        // }
        let server = request.into_inner().server.ok_or(tonic::Status::new(
            tonic::Code::InvalidArgument,
            "no server found in request",
        ))?;
        let child = Command::new(&self.steam_cmd)
            .args(vec![
                "+force_install_dir",
                Path::new(&self.base_dir)
                    .join(server.name)
                    .to_str()
                    .ok_or(tonic::Status::new(
                        tonic::Code::Internal,
                        format!("could not generate install path"),
                    ))?,
                "+login",
                "anonymous",
                "+app_update",
                &server.id.to_string(),
                "validate",
                "+exit",
            ])
            .spawn()
            .map_err(|e| Status::new(tonic::Code::Internal, format!("{}", e)))?;

        let child_id: i32 = child
            .id()
            .ok_or(tonic::Status::new(
                tonic::Code::Internal,
                "no process id returned",
            ))?
            .try_into()
            .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{}", e)))?;
        self.service_map
            .lock()
            .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{}", e)))?
            .insert(child_id, child);
        let reply = InstallServerReply {
            install_id: child_id,
        };
        Ok(Response::new(reply))
    }

    async fn start_server(
        &self,
        _request: tonic::Request<server::StartServerRequest>,
    ) -> Result<tonic::Response<server::StartServerReply>, tonic::Status> {
        println!("In start");
        // let tx = self.run_bus.clone();
        // if let Some(svr) = request.into_inner().server {
        //     tx.send(svr)
        //         .await
        //         .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{}", e)))?;
        // }
        let reply = StartServerReply { process_id: 1 };
        Ok(Response::new(reply))
    }
}
