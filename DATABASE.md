
## Running migrations outside the database container

Life is short. Write and run our migrations from the command line and connect
to the database with the regular tools like a human being.

Adapted from http://diesel.rs/guides/getting-started/

Let's create an initial migration

```sh
diesel setup   # we write out some initial files
diesel migration generate create_members
```

This makes blank up/down scripts. Write `CREATE TABLE` stuff in **up.sql** and
`DROP TABLE` stuff in **down.sql**.

We haven't needed to actually connect to the db yet, but now we're ready.

```sh
docker-compose up -d db  # start db on localhost:5432
source env.sh  # get our env
diesel migration run  # apply migrations
diesel migration redo  # test down/up
```

## View what Diesel thinks your schema is

This command connects to the DB and prints Rust macro code

```
diesel print-schema
```

Add it to **src/schema.rs**

```
diesel print-schema > src/schema.rs
```
