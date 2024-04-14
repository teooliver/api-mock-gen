ALTER TABLE task
ADD COLUMN user_id uuid REFERENCES "user"(id);
