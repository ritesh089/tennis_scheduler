-- Create the Player table to store player details.
CREATE TABLE IF NOT EXISTS players (
    player_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(100) UNIQUE NOT NULL,
    phone VARCHAR(100) DEFAULT NULL,
    role VARCHAR(50) DEFAULT 'player',
    password VARCHAR(255) NOT NULL, -- store hashed passwords
    skill_level VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- If creating the league table from scratch:

CREATE TABLE IF NOT EXISTS leagues (
    league_id SERIAL ,
    league_name VARCHAR(50) PRIMARY KEY NOT NULL UNIQUE,
    description TEXT,
    skill_level VARCHAR(50),
    created_by VARCHAR(50) NOT NULL,  -- References the player who created the league
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_creator
        FOREIGN KEY (created_by)
            REFERENCES players(name)
            ON DELETE CASCADE
);

-- Create the Appointment table to store match requests and scheduling.
CREATE TABLE IF NOT EXISTS appointment (
    appointment_id SERIAL PRIMARY KEY,
    requester_id VARCHAR(50) NOT NULL,
    opponent_id VARCHAR(50) NOT NULL,
    league_id VARCHAR(50), -- this is nullable for non-league matches
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    status VARCHAR(20) DEFAULT 'pending', -- possible values: pending, confirmed, canceled, declined
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_requester
        FOREIGN KEY(requester_id)
            REFERENCES players(name)
            ON DELETE CASCADE,
    CONSTRAINT fk_opponent
        FOREIGN KEY(opponent_id)
            REFERENCES players(name)
            ON DELETE CASCADE,
    CONSTRAINT fk_league
        FOREIGN KEY(league_id)
            REFERENCES leagues(league_name)
            ON DELETE SET NULL
);

--- If creating the join table from scratch:
CREATE TABLE IF NOT EXISTS player_leagues (
    player_id VARCHAR(50) NOT NULL,
    league_id VARCHAR(50) NOT NULL,
    role VARCHAR(20) DEFAULT 'player',  -- Role can be 'player' or 'admin'
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (player_id, league_id),
    singles_ranking INT,
    doubles_ranking INT,
    CONSTRAINT fk_player
        FOREIGN KEY(player_id)
            REFERENCES players(name)
            ON DELETE CASCADE,
    CONSTRAINT fk_league_join
        FOREIGN KEY(league_id)
            REFERENCES leagues(league_name)
            ON DELETE CASCADE
);

