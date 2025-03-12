### Run the app 
```
make run
```


# Run all tests
cargo test

# Run tests for a specific module
cargo test --test auth_tests
cargo test --test player_tests
cargo test --test appointment_tests
cargo test --test league_tests
cargo test --test match_tests

# Run a specific test
cargo test test_register
cargo test test_login

## API Endpoints

### Matches

#### Create a Match
- **URL**: `/api/matches`
- **Method**: `POST`
- **Description**: Create a new match
- **Request Body**:
  ```json
  {
    "match_type": "Singles",
    "player1_id": "1",
    "player2_id": "2",
    "league_id": "1",
    "team1_player1_id": null,
    "team1_player2_id": null,
    "team2_player1_id": null,
    "team2_player2_id": null,
    "datetime": "2023-05-15T14:00:00",
    "location": "Tennis Court 1",
    "status": "Scheduled",
    "notes": "Friendly match"
  }
  ```
- **Response**: 
  - Status: 201 Created
  - Body: 
    ```json
    {
      "message": "Match created successfully",
      "success": true
    }
    ```

#### Get Matches
- **URL**: `/api/matches`
- **Method**: `GET`
- **Description**: Get matches with optional filtering
- **Query Parameters**:
  - `league_id` (optional): Filter matches by league ID
  - `status` (optional): Filter matches by status (e.g., "Scheduled", "Completed", "Cancelled")
- **Response**: 
  - Status: 200 OK
  - Body: 
    ```json
    {
      "matches": [
        {
          "id": 1,
          "match_type": "Singles",
          "player1_id": "1",
          "player2_id": "2",
          "league_id": "1",
          "team1_player1_id": null,
          "team1_player2_id": null,
          "team2_player1_id": null,
          "team2_player2_id": null,
          "datetime": "2023-05-15T14:00:00",
          "location": "Tennis Court 1",
          "score": null,
          "winner_id": null,
          "status": "Scheduled",
          "notes": "Friendly match",
          "created_at": "2023-05-15T10:00:00"
        }
      ],
      "count": 1
    }
    ```

#### Get Player Matches
- **URL**: `/api/matches/player/{player_id}`
- **Method**: `GET`
- **Description**: Get all matches for a specific player (singles or doubles)
- **URL Parameters**:
  - `player_id`: ID of the player to get matches for
- **Response**: 
  - Status: 200 OK
  - Body: 
    ```json
    {
      "matches": [
        {
          "id": 1,
          "match_type": "Singles",
          "player1_id": "1",
          "player2_id": "2",
          "league_id": "1",
          "team1_player1_id": null,
          "team1_player2_id": null,
          "team2_player1_id": null,
          "team2_player2_id": null,
          "datetime": "2023-05-15T14:00:00",
          "location": "Tennis Court 1",
          "score": null,
          "winner_id": null,
          "status": "Scheduled",
          "notes": "Friendly match",
          "created_at": "2023-05-15T10:00:00"
        },
        {
          "id": 2,
          "match_type": "Doubles",
          "player1_id": null,
          "player2_id": null,
          "league_id": "1",
          "team1_player1_id": "1",
          "team1_player2_id": "3",
          "team2_player1_id": "4",
          "team2_player2_id": "5",
          "datetime": "2023-05-20T16:00:00",
          "location": "Tennis Court 2",
          "score": null,
          "winner_id": null,
          "status": "Pending",
          "notes": "Doubles match",
          "created_at": "2023-05-16T10:00:00"
        }
      ],
      "count": 2
    }
    ```
- **Error Responses**:
  - 500 Internal Server Error: If there's an issue with the database connection or query

#### Get Player Pending Matches
- **URL**: `/api/matches/pending/{player_id}`
- **Method**: `GET`
- **Description**: Get all pending matches for a specific player (singles or doubles)
- **URL Parameters**:
  - `player_id`: ID of the player to get pending matches for
- **Response**: 
  - Status: 200 OK
  - Body: 
    ```json
    {
      "matches": [
        {
          "id": 2,
          "match_type": "Doubles",
          "player1_id": null,
          "player2_id": null,
          "league_id": "1",
          "team1_player1_id": "1",
          "team1_player2_id": "3",
          "team2_player1_id": "4",
          "team2_player2_id": "5",
          "datetime": "2023-05-20T16:00:00",
          "location": "Tennis Court 2",
          "score": null,
          "winner_id": null,
          "status": "Pending",
          "notes": "Doubles match",
          "created_at": "2023-05-16T10:00:00"
        }
      ],
      "count": 1
    }
    ```
- **Error Responses**:
  - 500 Internal Server Error: If there's an issue with the database connection or query

#### Accept Match
- **URL**: `/api/matches/{match_id}/accept`
- **Method**: `POST`
- **Description**: Accepts a match request. The player must be in the league associated with the match. For doubles matches, all players (team1_player1, team1_player2, team2_player1, team2_player2) must be in the league.
- **URL Parameters**:
  - `match_id`: ID of the match to accept
- **Request Body**:
  ```json
  {
    "player_id": "string",
    "comments": "string (optional)"
  }
  ```
- **Response**: 
  - Status: 200 OK
  - Body: 
    ```json
    {
      "message": "Match accepted successfully",
      "success": true
    }
    ```
- **Error Responses**:
  - 404 Not Found: Match not found
  - 400 Bad Request: Player is not in the league or one of the players in a doubles match is not in the league 

#### Reject Match
- **URL**: `/api/matches/{match_id}/reject`
- **Method**: `POST`
- **Description**: Rejects a match request. No league membership verification is required.
- **URL Parameters**:
  - `match_id`: ID of the match to reject
- **Request Body**:
  ```json
  {
    "player_id": "string",
    "reason": "string (optional)"
  }
  ```
- **Response**: 
  - Status: 200 OK
  - Body: 
    ```json
    {
      "message": "Match rejected successfully",
      "success": true
    }
    ```
- **Error Responses**:
  - 404 Not Found: Match not found

## Running Tests

To run the tests, make sure you have set up the test database as described in the Test Database Setup section, then run:

```bash
cargo test -- --ignored
```

To run a specific test, use:

```bash
cargo test test_name -- --ignored
```

For example, to run the doubles match test:

```bash
cargo test test_accept_doubles_match -- --ignored
```

Or to run the reject match test:

```bash
cargo test test_reject_match -- --ignored
``` 