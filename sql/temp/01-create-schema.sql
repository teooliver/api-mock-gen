CREATE TABLE user (
    id uuid DEFAULT gen_random_uuid(),
    name VARCHAR(200),
    email VARCHAR(200),
    PRIMARY KEY (id),
);


CREATE TYPE TaskStatus AS ENUM (
    'Done',
    'InProgress',
    'NotNeeded',
    'ReadyToStart',
    'Backlog'
);

CREATE TABLE task (
    id uuid DEFAULT gen_random_uuid(),
    title VARCHAR(200),
    description VARCHAR(200),
    status TaskStatus
    user_ref uuid
    created_at TODO!
    updated_at TODO!
    finished_at TODO!
    color TODO!
)
