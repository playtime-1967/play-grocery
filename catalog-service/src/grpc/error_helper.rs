use tonic::Status;

pub fn handle(error: anyhow::Error) -> Status {
    //TODO: customize the response.
    Status::internal(error.to_string())
}
