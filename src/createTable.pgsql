-- Create Balance table
CREATE TABLE Balance (
    id SERIAL PRIMARY KEY,
    userid INTEGER NOT NULL,
    balance DOUBLE PRECISION NOT NULL
);

-- Create Transaction table
CREATE TABLE Transaction (
    id SERIAL PRIMARY KEY,
    purchase_type TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    userid INTEGER NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);
