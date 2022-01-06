pub mod sqlx_repository;

use crate::models::Task;
use crate::error::Result;

pub trait TaskRepository {
  async fn create_task(&self, title: String, decription: String) -> Result<Task>;
  async fn complete_task(&self, id: String) -> Result<Task>;
}
