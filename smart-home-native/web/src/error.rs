#[derive(Debug)]
pub enum ClientError {
    IncompleteHttpRequest,
    MissingMethod
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
        -> Result<(), std::fmt::Error> {
            write!(f, "{:?}", self)
        }
}

impl std::error::Error for ClientError {}