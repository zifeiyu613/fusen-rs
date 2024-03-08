use std::{collections::HashMap, sync::Arc};

use fusen_common::RpcServer;
use tokio::{
    net::TcpStream,
    sync::{broadcast, mpsc},
};
mod h2_handler;
pub mod server;

pub struct StreamHandler {
    pub tcp_stream: TcpStream,
    pub fusen_server: HashMap<String, Arc<Box<dyn RpcServer>>>,
    pub shutdown: broadcast::Receiver<()>,
    pub _shutdown_complete: mpsc::Sender<()>,
}
