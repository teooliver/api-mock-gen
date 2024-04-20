CREATE TABLE IF NOT EXISTS board (
id uuid DEFAULT gen_random_uuid(),
name VARCHAR(200),
PRIMARY KEY(id)
);
