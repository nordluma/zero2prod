-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '0a500b39-d193-42c1-b065-cfcca382d8b2',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$CL8/ab9s0GvNtpRNDpeFxA$2F1wfqZjRhNocem2BGFk8mwhmMgGpyg/n6eL53Zor7c'
)

