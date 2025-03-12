-- This file should undo anything in `up.sql`

-- Remove notes column from matches table
ALTER TABLE matches DROP COLUMN notes;
