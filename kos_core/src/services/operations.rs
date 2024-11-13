use crate::grpc_interface::google::longrunning::{
    operations_server::Operations, CancelOperationRequest, DeleteOperationRequest,
    GetOperationRequest, ListOperationsRequest, ListOperationsResponse, Operation as LroOperation,
    WaitOperationRequest,
};
use crate::hal::Operation;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub struct OperationsServiceImpl {
    operation_store: Arc<Mutex<HashMap<String, Operation>>>,
}

impl OperationsServiceImpl {
    pub fn new(operation_store: Arc<Mutex<HashMap<String, Operation>>>) -> Self {
        Self { operation_store }
    }
}

impl Default for OperationsServiceImpl {
    fn default() -> Self {
        unimplemented!(
            "Default is not implemented because OperationsServiceImpl requires external operations store"
        )
    }
}

#[tonic::async_trait]
impl Operations for OperationsServiceImpl {
    async fn get_operation(
        &self,
        request: Request<GetOperationRequest>,
    ) -> Result<Response<LroOperation>, Status> {
        let name = request.into_inner().name;
        let store = self.operation_store.lock().await;
        if let Some(operation) = store.get(&name) {
            Ok(Response::new(operation.clone()))
        } else {
            Err(Status::not_found("Operation not found"))
        }
    }

    // Implement other methods if needed
    async fn list_operations(
        &self,
        _request: Request<ListOperationsRequest>,
    ) -> Result<Response<ListOperationsResponse>, Status> {
        // Not implemented in this example
        Err(Status::unimplemented("ListOperations is not implemented"))
    }

    async fn cancel_operation(
        &self,
        _request: Request<CancelOperationRequest>,
    ) -> Result<Response<()>, Status> {
        // Not implemented in this example
        Err(Status::unimplemented("CancelOperation is not implemented"))
    }

    async fn delete_operation(
        &self,
        _request: Request<DeleteOperationRequest>,
    ) -> Result<Response<()>, Status> {
        // Not implemented in this example
        Err(Status::unimplemented("DeleteOperation is not implemented"))
    }

    async fn wait_operation(
        &self,
        _request: Request<WaitOperationRequest>,
    ) -> Result<Response<Operation>, Status> {
        // Not implemented in this example
        Err(Status::unimplemented("WaitOperation is not implemented"))
    }
}
