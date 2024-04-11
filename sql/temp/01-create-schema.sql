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
