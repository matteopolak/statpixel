-- This file should undo anything in `up.sql`

ALTER TABLE schedule
	DROP COLUMN hash;
