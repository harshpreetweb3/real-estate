-- Your SQL goes here
CREATE TABLE PropertyTypes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE PropertyCategories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);