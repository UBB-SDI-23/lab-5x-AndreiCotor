pub mod users_repo;
pub mod submission_repository;
pub mod contest_repository;
pub mod participates_repository;
pub mod problem_repository;
pub mod user_credentials_repo;

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
type DbError = Box<dyn std::error::Error + Send + Sync>;
type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

