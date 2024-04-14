ALTER TABLE task
ADD COLUMN status_id uuid REFERENCES "status"(id);
