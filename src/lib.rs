pub mod server {
    tonic::include_proto!("server");
}
use std::{path::Path, process::Stdio};

use server::{server_service_server::ServerService, ServerInstallOutput, TestStreamResponse};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::mpsc,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};

pub struct Service {
    // install_bus: Sender<server::Server>,
    // run_bus: Sender<server::Server>,
    //service_map: Arc<Mutex<HashMap<i32, Child>>>,
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
            //service_map: Arc::new(Mutex::new(HashMap::new())),
            steam_cmd: steam_cmd.into(),
            base_dir: base_dir.into(),
        }
    }
}

#[tonic::async_trait]
impl ServerService for Service {
    type InstallServerStream = ReceiverStream<Result<ServerInstallOutput, Status>>;

    async fn install_server(
        &self,
        request: tonic::Request<server::InstallServerRequest>,
    ) -> Result<tonic::Response<Self::InstallServerStream>, tonic::Status> {
        let server = request.into_inner().server.ok_or(tonic::Status::new(
            tonic::Code::InvalidArgument,
            "no server found in request",
        ))?;
        let (tx, rx) = mpsc::channel(4);
        let mut child = Command::new(&self.steam_cmd)
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
            //.kill_on_drop(true)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| Status::new(tonic::Code::Internal, format!("{}", e)))?;

        tokio::spawn(async move {
            let mut f = BufReader::new(child.stdout.take().unwrap()).lines();
            while let Some(line) = f.next_line().await.unwrap() {
                tx.send(Ok(ServerInstallOutput { msg: line }))
                    .await
                    .unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type TestStreamStream = ReceiverStream<Result<TestStreamResponse, Status>>;

    async fn test_stream(
        &self,
        request: tonic::Request<server::TestStreamRequest>,
    ) -> Result<tonic::Response<Self::TestStreamStream>, tonic::Status> {
        let process = request.into_inner().process_location;
        let (tx, rx) = mpsc::channel(4);
        let mut child = Command::new(&process)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        tokio::spawn(async move {
            let mut f = BufReader::new(child.stdout.take().unwrap()).lines();
            while let Some(line) = f.next_line().await.unwrap() {
                tx.send(Ok(TestStreamResponse { msg: line })).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn start_server(
        &self,
        _request: tonic::Request<server::StartServerRequest>,
    ) -> Result<tonic::Response<server::StartServerReply>, tonic::Status> {
        todo!()
    }
}

// #[tonic::async_trait]
// impl ServerService for Service {
//     async fn install_server(
//         &self,
//         request: tonic::Request<server::InstallServerRequest>,
//     ) -> Result<tonic::Response<Self::GetServerInstallStream>, tonic::Status> {
//         println!("In install");
//         // let tx = self.install_bus.clone();
//         // if let Some(svr) = request.into_inner().server {
//         //     tx.send(svr)
//         //         .await
//         //         .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{}", e)))?;
//         // }
//         let server = request.into_inner().server.ok_or(tonic::Status::new(
//             tonic::Code::InvalidArgument,
//             "no server found in request",
//         ))?;
//         let child = Command::new(&self.steam_cmd)
//             .args(vec![
//                 "+force_install_dir",
//                 Path::new(&self.base_dir)
//                     .join(server.name)
//                     .to_str()
//                     .ok_or(tonic::Status::new(
//                         tonic::Code::Internal,
//                         format!("could not generate install path"),
//                     ))?,
//                 "+login",
//                 "anonymous",
//                 "+app_update",
//                 &server.id.to_string(),
//                 "validate",
//                 "+exit",
//             ])
//             .kill_on_drop(true)
//             .stdout(Stdio::piped())
//             .stderr(Stdio::piped())
//             .spawn()
//             .map_err(|e| Status::new(tonic::Code::Internal, format!("{}", e)))?;

//         let stdout = child.stdout.ok_or(tonic::Status::new(
//             tonic::Code::Internal,
//             "could not get stdout for process",
//         ))?;

//         let (tx, rx) = mpsc::channel(4);

//         tokio::spawn(async move {
//             println!("in streaming response");
//             let mut lines = BufReader::new(stdout).lines();
//             // TODO: figure out the unwraps here
//             while let Some(l) = lines.next_line().await.unwrap() {
//                 println!("Sending line: {}", l);
//                 tx.send(Ok(ServerInstallOutput { msg: l })).await.unwrap();
//             }
//         });

//         Ok(Response::new(ReceiverStream::new(rx)))
//     }

//     async fn start_server(
//         &self,
//         _request: tonic::Request<server::StartServerRequest>,
//     ) -> Result<tonic::Response<server::StartServerReply>, tonic::Status> {
//         println!("In start");
//         // let tx = self.run_bus.clone();
//         // if let Some(svr) = request.into_inner().server {
//         //     tx.send(svr)
//         //         .await
//         //         .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{}", e)))?;
//         // }
//         let reply = StartServerReply { process_id: 1 };
//         Ok(Response::new(reply))
//     }

//     type GetServerInstallStream = ReceiverStream<Result<ServerInstallOutput, Status>>;

//     async fn get_server_install(
//         &self,
//         request: tonic::Request<server::GetServerInstallRequest>,
//     ) -> Result<tonic::Response<Self::GetServerInstallStream>, tonic::Status> {
//         let id = request.into_inner().install_id;
//         let mut child_map = self
//             .service_map
//             .lock()
//             .map_err(|e| tonic::Status::new(tonic::Code::Internal, format!("{}", e)))?;

//         let child = child_map.remove(&id).ok_or(tonic::Status::new(
//             tonic::Code::Internal,
//             "no child process with id found",
//         ))?;
//         let stdout = child.stdout.ok_or(tonic::Status::new(
//             tonic::Code::Internal,
//             "could not get stdout for process",
//         ))?;

//         let (tx, rx) = mpsc::channel(4);

//         tokio::spawn(async move {
//             let mut lines = BufReader::new(stdout).lines();
//             // TODO: figure out the unwraps here
//             while let Some(l) = lines.next_line().await.unwrap() {
//                 tx.send(Ok(ServerInstallOutput { msg: l })).await.unwrap();
//             }
//         });

//         Ok(Response::new(ReceiverStream::new(rx)))
//     }

//     type InstallServerStream = ReceiverStream<Result<ServerInstallOutput, Status>>;
// }
