use std::sync::Once;
use tennis_scheduler::db::DbPool;

// Import the setup_test_db module using a relative path
#[path = "setup_test_db.rs"]
pub mod setup_test_db;

static INIT: Once = Once::new();
static mut TEST_DB_POOL: Option<DbPool> = None;

/// Initialize the test database once for all tests
pub fn initialize() -> DbPool {
    unsafe {
        INIT.call_once(|| {
            let (pool, _) = setup_test_db::create_test_db_pool();
            TEST_DB_POOL = Some(pool);
        });
        
        TEST_DB_POOL.clone().unwrap()
    }
}

/// Get a connection pool for the test database
pub fn get_test_db_pool() -> DbPool {
    initialize()
} 