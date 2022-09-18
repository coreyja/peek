-- Add migration script here
ALTER TABLE "Sessions"
ADD COLUMN "user_id" integer
REFERENCES "Users"(id)
