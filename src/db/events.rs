use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

pub enum UserEvents {
    JoinedVocalChannel(i64, String, bool, i64, i64),
    LeftVocalChannel(i64),
    SentText(i64, String, bool, i64, i64),
}

#[async_trait]
pub trait Observer {
    async fn notify(&self, mut rx: UnboundedReceiver<UserEvents>);
}
