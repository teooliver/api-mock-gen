CREATE TABLE IF NOT EXISTS "task" (
id uuid DEFAULT gen_random_uuid(),
title VARCHAR(200),
description VARCHAR(200),
status VARCHAR(200),
color VARCHAR(100),
PRIMARY KEY(id)
);
