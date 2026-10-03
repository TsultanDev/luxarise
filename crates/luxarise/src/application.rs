pub struct Application {
    running: bool,
}
impl Application {
    pub fn init() -> Application {
        tracing::info!("Init Application!!");

        Application { running: true }
    }
}
