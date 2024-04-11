INSERT INTO user (username) VALUES ('demo1')

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
