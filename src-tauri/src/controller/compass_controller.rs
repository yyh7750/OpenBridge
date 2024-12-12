use crate::model::compass::Compass;
use crate::service::compass_socket::{receive_data, socket_init};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;

pub struct AppState {
    pub socket: Arc<UdpSocket>, // Arc로 감싸 안전하게 공유
    pub data: Arc<RwLock<Option<Compass>>>, // 최신 데이터를 저장하는 상태
}

impl AppState {
    pub fn new(socket: Arc<UdpSocket>) -> Self {
        Self {
            socket,
            data: Arc::new(RwLock::new(None)),
        }
    }
}

pub async fn start_udp_service(bind_addr: &str) -> Result<Arc<AppState>, String> {
    let socket = match socket_init(bind_addr).await {
        Ok(sock) => Arc::new(sock), // Arc로 소켓 감싸기
        Err(e) => return Err(format!("Failed to initialize socket: {}:{}: {}", file!(), line!(), e)),
    };

    let app_state = Arc::new(AppState::new(Arc::clone(&socket)));

    let app_state_clone = Arc::clone(&app_state);
    tokio::spawn(async move {
        loop {
            if let Some(received_data) = receive_data(&app_state_clone.socket).await {
                // 수신된 데이터를 상태에 저장
                let mut shared_data = app_state_clone.data.write().await;
                *shared_data = Some(received_data);
                // println!("Data updated in state.");
            }
        }
    });

    Ok(app_state)
}
