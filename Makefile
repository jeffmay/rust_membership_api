default: dev

# Build and package all the images
build:
	./container.sh

# Run the API server locally outside of a container
dev:
	dotenv-shell cargo run

# Load all infrastructure (databases, caches, etc)
load:
	docker-compose up -d db

# Run all migrations
migrate:
	dotenv-shell diesel migration run

# Dump the schema from the database
schema:
	diesel print-schema > src/db/schema.rs
