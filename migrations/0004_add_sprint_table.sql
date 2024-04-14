CREATE TABLE IF NOT EXISTS sprint (
id uuid DEFAULT gen_random_uuid(),
name VARCHAR(200),
PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS status (
id uuid DEFAULT gen_random_uuid(),
name VARCHAR(200),
sprint_id uuid REFERENCES sprint(id),
PRIMARY KEY(id)
);
