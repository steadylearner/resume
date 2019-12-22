-- http://www.postgresqltutorial.com/postgresql-uuid/
-- CREATE DATABASE grpc OWNER you;
-- \c grpc;
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- SELECT uuid_generate_v4();

CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    first_name VARCHAR(60) NOT NULL,
    last_name VARCHAR(60) NOT NULL,
    date_of_birth Date NOT NULL
);

INSERT INTO users
    (first_name, last_name, date_of_birth)
VALUES
    ('steady', 'learner', '2019-01-01');

INSERT INTO users
    (first_name, last_name, date_of_birth)
VALUES
    ('mybirthdayis', 'blackfriday', '2019-11-25');

INSERT INTO users
    (first_name, last_name, date_of_birth)
VALUES
    ('mybirthdayis', 'notblackfriday', '2019-11-26');
