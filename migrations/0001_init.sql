CREATE TABLE user (
    id uuid DEFAULT gen_random_uuid(),
    name VARCHAR(200),
    email VARCHAR(200),
    PRIMARY KEY (id),
);


DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'taskstatus') THEN
      CREATE TYPE TaskStatus AS ENUM (
    	'Done',
    	'InProgress',
    	'NotNeeded',
    	'ReadyToStart',
    	'Backlog'
);
    END IF;
END$$;


CREATE TABLE IF NOT EXISTS task (
id uuid DEFAULT gen_random_uuid(),
title VARCHAR(200),
description VARCHAR(200),
status TaskStatus,
user_ref uuid,
created_at timestamp with time zone,
updated_at timestamp with time zone,
finished_at timestamp with time zone,
color VARCHAR(100)
);

-- INSERT INTO task (
-- title,
-- description,
-- status,
-- color
-- )
-- VALUES (
-- 'SomeTitle',
-- 'Description',
-- 'Backlog',
-- ''
-- );
