mod actuator;
mod firmware;

pub use actuator::*;
pub use firmware::*;

pub struct ZBotPlatform {}

impl ZBotPlatform {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ZBotPlatform {
    fn default() -> Self {
        ZBotPlatform::new()
    }
}

#[async_trait]
impl Platform for ZBotPlatform {
    fn name(&self) -> &'static str {
        "ZBot"
    }

    fn serial(&self) -> String {
        "00000000".to_string()
    }

    fn initialize(&mut self, _operations_service: Arc<OperationsServiceImpl>) -> eyre::Result<()> {
        Ok(())
    }

    fn create_services<'a>(
        &'a mut self,
        operations_service: Arc<OperationsServiceImpl>,
    ) -> Pin<Box<dyn Future<Output = eyre::Result<Vec<ServiceEnum>>> + Send + 'a>> {
        Box::pin(async move {
            let actuator = ZBotActuator::new(operations_service);
            Ok(vec![ServiceEnum::Actuator(Box::new(actuator))])
        })
    }
}
