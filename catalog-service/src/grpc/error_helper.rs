use tonic::Status;

pub fn handle(error: anyhow::Error) -> Status {
    //TODO: Customize your response here
    Status::internal(error.to_string())
}
