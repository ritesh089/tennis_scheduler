# Tennis Scheduler Tests

This directory contains integration tests for the Tennis Scheduler application.

## Test Structure

- `auth_tests.rs`: Tests for authentication functionality (register, login)
- `player_tests.rs`: Tests for player-related endpoints
- `league_tests.rs`: Tests for league management functionality

## Running Tests

The tests are marked with `#[ignore]` so they don't run by default. This is because they require a database connection and can take some time to run.

To run all tests:

```bash
cargo test -- --ignored
```

To run a specific test file:

```bash
cargo test --test auth_tests -- --ignored
cargo test --test player_tests -- --ignored
cargo test --test league_tests -- --ignored
```

To run a specific test:

```bash
cargo test test_register -- --ignored
cargo test test_login -- --ignored
```

## Test Database Setup

The tests use a real database connection. You need to have PostgreSQL installed and running on your machine.

### Prerequisites

1. Install PostgreSQL if you haven't already:
   - macOS: `brew install postgresql` and `brew services start postgresql`
   - Ubuntu: `sudo apt-get install postgresql` and `sudo service postgresql start`
   - Windows: Download and install from the [PostgreSQL website](https://www.postgresql.org/download/windows/)

2. Create a test database:
   ```bash
   createdb tennis_scheduler_test
   ```

3. Run the migrations on the test database:
   ```bash
   DATABASE_URL=postgres://username:password@localhost/tennis_scheduler_test diesel migration run
   ```

4. Make sure your `.env` file contains the correct database URL:
   ```
   DATABASE_URL=postgres://username:password@localhost/tennis_scheduler_test
   ```

Each test file includes a `setup_test_db()` function that creates a connection pool to the database specified in your `.env` file.

### Troubleshooting

If you encounter connection issues:

1. Check if PostgreSQL is running:
   ```bash
   # macOS
   brew services list
   
   # Ubuntu
   sudo service postgresql status
   
   # Windows
   # Check Services in Task Manager
   ```

2. Verify your database URL:
   - Make sure the username and password are correct
   - Check that the database name exists
   - Ensure PostgreSQL is running on the specified port (default is 5432)

3. Test the connection manually:
   ```bash
   psql postgres://username:password@localhost/tennis_scheduler_test
   ```

## Test Data

The tests create their own test data using unique names and emails for each test run. This ensures that tests don't interfere with each other and can be run multiple times without issues.

## Test Coverage

The tests cover the following functionality:

### Authentication
- User registration
- User login
- Handling duplicate email registrations

### Players
- Getting player information
- Updating player information
- Getting all players
- Searching for players

### Leagues
- Creating leagues
- Getting all leagues
- Joining leagues
- Getting league players

## Adding New Tests

When adding new tests, follow these guidelines:

1. Create a new test file if testing a new module
2. Use the `setup_test_db()` function to get a database connection
3. Register any required users before testing functionality
4. Use the helper functions `unique_name()` and `unique_email()` to generate unique test data
5. Use descriptive test names that indicate what is being tested
6. Add assertions to verify the expected behavior
7. Mark the tests with `#[ignore]` so they don't run by default 