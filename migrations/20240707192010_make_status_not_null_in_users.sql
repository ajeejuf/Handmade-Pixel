BEGIN;
    UPDATE users
        SET status = 'confirmed'
        WHERE status IS NULL;
    ALTER TABLE users ALTER COLUMN status SET NOT NULL;
COMMIT;
