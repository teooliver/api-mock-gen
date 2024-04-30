-- Add migration script here
ALTER TABLE IF EXISTS "user"
RENAME TO "app_user";
