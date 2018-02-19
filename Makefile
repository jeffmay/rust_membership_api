default: dev

# Build and package all the images
build:
	./container.sh

# Run the API server locally outside of a container
dev: load
	dotenv-shell cargo run

# Load all infrastructure (databases, caches, etc)
load:
	docker-compose up -d db
