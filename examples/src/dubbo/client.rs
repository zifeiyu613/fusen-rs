use examples::{DemoServiceClient, ReqDto};
use fusen::client::FusenClient;
use fusen::fusen_common;
use fusen::fusen_common::url::UrlConfig;
use fusen::register::nacos::NacosConfig;
use lazy_static::lazy_static;
use tracing::info;

lazy_static! {
    static ref CLI: FusenClient = FusenClient::build(
        NacosConfig::builder()
            .server_addr("127.0.0.1:8848".to_owned())
            .app_name(Some("dubbo-client".to_owned()))
            .server_type(fusen::register::Type::Dubbo)
            .build()
            .boxed()
    );
}

#[tokio::main(worker_threads = 512)]
async fn main() {
    fusen_common::logs::init_log();
    let client = DemoServiceClient::new(&CLI);
    info!("{:?}", client.get_info());
    let res = client
        .sayHelloV2(ReqDto {
            str: "world".to_string(),
        })
        .await;
    info!("{:?}", res);
}