use failure::Fail;

#[derive(Debug, Fail)]
pub enum TodoAppError {
  #[fail(display = "Sqlx error: {}", _0)]
  SqlxError(sqlx::Error)
}

pub type Result<T> = std::result::Result<T, TodoAppError>;

impl From<sqlx::Error> for TodoAppError {
  fn from(e: sqlx::Error) -> Self {
      TodoAppError::SqlxError(e)
  }
}
