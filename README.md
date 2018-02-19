# Contributing

## Installation

### Nightly Rust

Use `rustup` to install Rust language dependencies. Install it [here](https://www.rustup.rs/) 

```bash
rustup default nightly
```

### Dotenv Shell

We accept all configuration as environment variables. This works fine when you have a build process
that can inject all the environment variables needed based on some deployment config file, however 
when you are developing locally, you'll want a simple way to load the environment variables into
the app.

The `dotenv-shell` script will allow you to run anything from your shell using the `.env` file in
the current working directory.

```bash
cargo install dotenv-shell
```

You'll also need to set up your local `.env` file.

```bash
cp example.env .env
# edit .env with any custom options for your environment
```

### Postgres Libraries

Install Postgres SQL libraries and / or admin GUI. Although we run the database in a container,
we need the libraries for compiling rust dependencies.

**Mac OS X**

* I recommend downloading and installing [PSequel](http://www.psequel.com/) for a GUI
* If you don't want the GUI, you can just [download the Postgres Server](https://www.postgresql.org/)

**Arch Linux**

* You are using Arch Linux, so I assume you probably don't want a GUI. To get just the libraries:
```bash
sudo pacman -S postgresql-libs
```

### Diesel CLI

Diesel is our database ORM and migration framework. Install with only Postgres features to avoid
needing MySQL libraries.

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### Docker Compose

We use `docker-compose` to run the Postgres database in a container. This keeps the database 
separate from any globally installed versions and configs to avoid confusion.

Almost all rust development can happen locally without `docker` by using simple `cargo` 
commands (`make` just forwards to `cargo` to run the app). However, we also use `docker-compose` 
for testing our membership API docker image after we build it.

You can find the installation instructions [here](https://docs.docker.com/compose/install/)

## Running the database

We run the database using `docker-compose`.

```bash
docker-compose up -d db  # start db on localhost:5432
```

## Running the API server

To run the server locally, make sure you have gone through all the pre-requisites in the 
Installation section, then just run:

```bash
make
# This is basically just an alias for
# dotenv-shell cargo run
```

It's easy enough to just call `cargo` directly, but sometimes it can be tricky to know when you 
need to run a command with the `.env` variables. Using `make` commands will ensure that you are 
running the command in a way that will work with the current version of the code.

## Running migrations

[See [Diesel's Getting Started Guide](http://diesel.rs/guides/getting-started/) for more info]

We use diesel to run migrations. The easiest way is to run:

```bash
make migrate
# This is basically just an alias for
# dotenv-shell diesel migration run
```

If you want to run more custom `diesel-cli` commands, you'll need to remember to run the commands 
with `dotenv-shell` to get the required environment variables into diesel so it can connect.

### Synchronizing the schema with the code

This command connects to the DB and prints Rust macro code based on the schema in your database.

```bash
diesel print-schema
```

Add it to **src/db/schema.rs** to keep this file up-to-date with the migrated database schema.

```bash
diesel print-schema > src/db/schema.rs
```

## Building and testing the Docker image

When you want to test the image running in docker, you must first build the image with:

```bash
make build
# This is basically just an alias for
# ./container.sh 
```

This will build the container so that you can deploy the app in docker with:

```bash
docker-compose up api
```

Running with `docker-compose` doesn't require using `dotenv` since it support `.env` files natively.
You cannot use `docker-compose` to build the image. This is to avoid code bloat from packaging all 
the build environment and tools into the artifact with what is otherwise a simple binary.
