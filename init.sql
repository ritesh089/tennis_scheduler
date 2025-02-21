-- Create the Player table to store player details.
CREATE TABLE IF NOT EXISTS player (
    player_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL, -- store hashed passwords
    skill_level VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create the League table for league information.
CREATE TABLE IF NOT EXISTS league (
    league_id SERIAL PRIMARY KEY,
    league_name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create the Appointment table to store match requests and scheduling.
CREATE TABLE IF NOT EXISTS appointment (
    appointment_id SERIAL PRIMARY KEY,
    requester_id INT NOT NULL,
    opponent_id INT NOT NULL,
    league_id INT, -- this is nullable for non-league matches
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    status VARCHAR(20) DEFAULT 'pending', -- possible values: pending, confirmed, canceled, declined
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_requester
        FOREIGN KEY(requester_id)
            REFERENCES player(player_id)
            ON DELETE CASCADE,
    CONSTRAINT fk_opponent
        FOREIGN KEY(opponent_id)
            REFERENCES player(player_id)
            ON DELETE CASCADE,
    CONSTRAINT fk_league
        FOREIGN KEY(league_id)
            REFERENCES league(league_id)
            ON DELETE SET NULL
);

-- Create a join table for the many-to-many relationship between players and leagues.
CREATE TABLE IF NOT EXISTS player_league (
    player_id INT NOT NULL,
    league_id INT NOT NULL,
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (player_id, league_id),
    CONSTRAINT fk_player
        FOREIGN KEY(player_id)
            REFERENCES player(player_id)
            ON DELETE CASCADE,
    CONSTRAINT fk_league_join
        FOREIGN KEY(league_id)
            REFERENCES league(league_id)
            ON DELETE CASCADE
);
