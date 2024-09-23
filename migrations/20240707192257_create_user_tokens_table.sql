
CREATE TABLE user_tokens(
    user_token TEXT NOT NULL,
    user_id uuid NOT NULL REFERENCES users (id),
    PRIMARY KEY (user_token)
);
