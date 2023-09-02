-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '784b2000-6cdb-4d07-881f-918212c9055d',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$4pGWljZSiRSsNZKSl4vVIw$8JjrymbJFWd5T1daronoOtfOtvoS0i7KY5E67I2AdV4'
)
