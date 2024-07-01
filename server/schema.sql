-- PostgreSQL 16 --

-- TODO add the uid and add a name and description column to the graph table.

-- TODO? add an input and output types too for embedding other graphs.
-- TODO?Possible alternative is just importing all the nodes and put them in a box instead for safety

CREATE TABLE IF NOT EXISTS users 
    (id SERIAL PRIMARY KEY,
    username TEXT,
    email TEXT,
    password TEXT,
    referral TEXT,
    creation_date TIMESTAMPTZ);

-- INSERT INTO users (username, email, password) VALUES ('admin', 'elhamaryanpur', 'password');

-- uid INT REFERENCES accounts (id), -- 
CREATE TABLE IF NOT EXISTS graph 
    (id SERIAL PRIMARY KEY, 
    content JSONB[], 
    generated_javascript TEXT, 
    creation_date TIMESTAMPTZ);