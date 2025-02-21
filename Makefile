.PHONY: run test-register test-login test-get-calendar

# Build and run the Docker containers
run:
	docker-compose up --build

# Test the registration endpoint
test-register:
	curl -X POST http://localhost:8080/api/register \
		-H "Content-Type: application/json" \
		-d '{"name": "Test User", "email": "test@example.com", "password": "password123", "skill_level": "intermediate"}'

# Test the login endpoint
test-login:
	curl -X POST http://localhost:8080/api/login \
		-H "Content-Type: application/json" \
		-d '{"email": "test@example.com", "password": "password123"}'

# Test fetching a player's calendar (using player_id 1 as an example)
test-get-calendar:
	curl http://localhost:8080/api/players/1/calendar
