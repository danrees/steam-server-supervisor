pub mod server {
    tonic::include_proto!("server");
}
use server::{server_service_server::ServerService, InstallServerReply, StartServerReply};
use tokio::sync::mpsc::Sender;
use tonic::Response;

pub struct Service {
    install_bus: Sender<server::Server>,
    run_bus: Sender<server::Server>,
}

impl Service {
    pub fn new(install_bus: Sender<server::Server>, run_bus: Sender<server::Server>) -> Self {
        Service {
            install_bus,
            run_bus,
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
        let tx = self.install_bus.clone();
        if let Some(svr) = request.into_inner().server {
            tx.send(svr)
                .await
                .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{}", e)))?;
        }
        let reply = InstallServerReply { install_id: 1 };
        Ok(Response::new(reply))
    }

    async fn start_server(
        &self,
        request: tonic::Request<server::StartServerRequest>,
    ) -> Result<tonic::Response<server::StartServerReply>, tonic::Status> {
        println!("In start");
        let tx = self.run_bus.clone();
        if let Some(svr) = request.into_inner().server {
            tx.send(svr)
                .await
                .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{}", e)))?;
        }
        let reply = StartServerReply { process_id: 1 };
        Ok(Response::new(reply))
    }
}
