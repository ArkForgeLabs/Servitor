-- PostgreSQL 16 --

CREATE TABLE IF NOT EXISTS users 
    (id SERIAL PRIMARY KEY, 
    username TEXT, 
    email TEXT, 
    password TEXT, 
    creation_date TIMESTAMP);

-- INSERT INTO users (username, email, password) VALUES ('admin', 'elhamaryanpur', 'password');

-- uid INT REFERENCES accounts (id), -- 
CREATE TABLE IF NOT EXISTS graph 
    (id SERIAL PRIMARY KEY, 
    content JSONB[], 
    generated_javascript TEXT, 
    creation_date TIMESTAMPTZ);