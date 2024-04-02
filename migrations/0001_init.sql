CREATE TABLE user (
    id uuid DEFAULT gen_random_uuid(),
    name VARCHAR(200),
    email VARCHAR(200),
    PRIMARY KEY (id),
);
