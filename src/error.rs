pub type TONAPIResult<R, E = TONAPIError> = Result<R, E>;

#[derive(Debug)]
pub enum TONAPIError {
    GlobalConfigError(String),
}
