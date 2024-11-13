use crate::grpc_interface::google::longrunning::{
    operations_server::Operations, CancelOperationRequest, DeleteOperationRequest,
    GetOperationRequest, ListOperationsRequest, ListOperationsResponse, Operation as LroOperation,
    WaitOperationRequest,
};
use crate::hal::Operation;
use prost::Message;
use prost_types::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::async_trait;
use tonic::{Request, Response, Status};

pub struct OperationsServiceImpl {
    pub operation_store: Arc<Mutex<HashMap<String, Operation>>>,
}

impl OperationsServiceImpl {
    pub fn new(operation_store: Arc<Mutex<HashMap<String, Operation>>>) -> Self {
        Self { operation_store }
    }

    pub async fn create<T: Message>(
        &self,
        name: String,
        metadata: T,
        type_url: &str,
    ) -> Result<LroOperation, Status> {
        let mut buf = Vec::new();
        metadata
            .encode(&mut buf)
            .map_err(|e| Status::internal(format!("Failed to encode metadata: {}", e)))?;

        let operation = LroOperation {
            name: name.clone(),
            metadata: Some(Any {
                type_url: type_url.to_string(),
                value: buf,
            }),
            done: false,
            result: None,
        };

        self.operation_store
            .lock()
            .await
            .insert(name, operation.clone());

        Ok(operation)
    }

    pub async fn get_metadata<T: Message + Default>(
        &self,
        name: &str,
    ) -> Result<Option<T>, Status> {
        let store = self.operation_store.lock().await;
        if let Some(operation) = store.get(name) {
            if let Some(metadata) = &operation.metadata {
                return T::decode(&metadata.value[..])
                    .map(Some)
                    .map_err(|e| Status::internal(format!("Failed to decode metadata: {}", e)));
            }
        }
        Ok(None)
    }

    pub async fn update_metadata<T: Message>(
        &self,
        name: &str,
        metadata: T,
        mark_done: bool,
    ) -> Result<(), Status> {
        let mut store = self.operation_store.lock().await;

        if let Some(operation) = store.get_mut(name) {
            let mut buf = Vec::new();
            metadata
                .encode(&mut buf)
                .map_err(|e| Status::internal(format!("Failed to encode metadata: {}", e)))?;

            if let Some(existing_metadata) = &mut operation.metadata {
                existing_metadata.value = buf;
                if mark_done {
                    operation.done = true;
                }
                Ok(())
            } else {
                Err(Status::internal("Operation has no metadata field"))
            }
        } else {
            Err(Status::not_found("Operation not found"))
        }
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

#[async_trait]
impl Operations for Arc<OperationsServiceImpl> {
    async fn get_operation(
        &self,
        request: Request<GetOperationRequest>,
    ) -> Result<Response<Operation>, Status> {
        self.as_ref().get_operation(request).await
    }

    async fn list_operations(
        &self,
        request: Request<ListOperationsRequest>,
    ) -> Result<Response<ListOperationsResponse>, Status> {
        self.as_ref().list_operations(request).await
    }

    async fn delete_operation(
        &self,
        request: Request<DeleteOperationRequest>,
    ) -> Result<Response<()>, Status> {
        self.as_ref().delete_operation(request).await
    }

    async fn cancel_operation(
        &self,
        request: Request<CancelOperationRequest>,
    ) -> Result<Response<()>, Status> {
        self.as_ref().cancel_operation(request).await
    }

    async fn wait_operation(
        &self,
        request: Request<WaitOperationRequest>,
    ) -> Result<Response<Operation>, Status> {
        self.as_ref().wait_operation(request).await
    }
}
