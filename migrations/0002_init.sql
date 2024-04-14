CREATE TABLE IF NOT EXISTS "user" (
id uuid DEFAULT gen_random_uuid(),
email VARCHAR(200) NOT NULL UNIQUE,
first_name VARCHAR(200),
last_name VARCHAR(200),
PRIMARY KEY(id)
);
