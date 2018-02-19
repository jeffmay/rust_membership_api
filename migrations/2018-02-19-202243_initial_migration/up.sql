-- Creates all the tables and indexes

CREATE TABLE "committees" (
    "id" SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL
);

-- Diesel doesn't handle enums well
-- CREATE TYPE ELECTION_STATUS AS ENUM ('draft', 'published', 'final');

CREATE TABLE "elections" (
    "id" SERIAL PRIMARY KEY,
    "name" TEXT NOT NULL UNIQUE,
    "status" TEXT NOT NULL,
    "number_winners" int NOT NULL,
    "voting_begins" timestamp,
    "voting_ends" timestamp
);

CREATE TABLE "meetings" (
    "id" SERIAL PRIMARY KEY,
    "short_id" int UNIQUE,
    "name" TEXT NOT NULL,
    "committee_id" int REFERENCES "committees" ("id"),
    "start_time" timestamp,
    "end_time" timestamp
);

CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "email_address" TEXT,
    "normalized_email" TEXT,
    "first_name" TEXT,
    "last_name" TEXT,
    "biography" TEXT
);

CREATE TABLE "attendees" (
    "id" SERIAL PRIMARY KEY,
    "meeting_id" int NOT NULL REFERENCES "meetings" ("id"),
    "user_id" int NOT NULL REFERENCES "users" ("id")
);

CREATE TABLE "candidates" (
  "id" SERIAL PRIMARY KEY,
  "election_id" INT NOT NULL REFERENCES "elections" ("id"),
  "name" TEXT NOT NULL,
  "image_url" TEXT
);

CREATE TABLE "eligible_voters" (
  "id" SERIAL PRIMARY KEY,
  "user_id" INT NOT NULL REFERENCES "users" ("id"),
  "voted" BOOLEAN NOT NULL,
  "election_id" INT NOT NULL REFERENCES "elections" ("id"),
  UNIQUE ("user_id", "election_id")
);

CREATE TABLE "emails" (
  "id" SERIAL PRIMARY KEY,
  "external_id" TEXT NOT NULL UNIQUE,
  "email_address" TEXT NOT NULL UNIQUE,
  "committee_id" INT REFERENCES "committees" ("id")
);

CREATE TABLE "roles" (
    "id" SERIAL PRIMARY KEY,
    "committee_id" int REFERENCES "committees" ("id"),
    "user_id" int NOT NULL REFERENCES "users" ("id"),
    "role" TEXT NOT NULL
);

CREATE TABLE "votes" (
  "id" SERIAL PRIMARY KEY,
  "election_id" INT NOT NULL REFERENCES "elections" ("id"),
  "vote_key" INT NOT NULL UNIQUE,
  UNIQUE ("election_id", "vote_key")
);

CREATE TABLE "forwarding_addresses" (
  "id" SERIAL PRIMARY KEY,
  "forward_to" TEXT NOT NULL,
  "incoming_email_id" INT NOT NULL REFERENCES "emails" ("id")
);

CREATE TABLE "rankings" (
  "id" SERIAL PRIMARY KEY,
  "candidate_id" INT NOT NULL REFERENCES "candidates" ("id"),
  "vote_id" INT NOT NULL REFERENCES "votes" ("id"),
  "rank" INT NOT NULL
);
