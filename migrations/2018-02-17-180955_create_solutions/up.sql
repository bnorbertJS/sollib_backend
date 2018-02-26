-- Your SQL goes here
CREATE TABLE solutions (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  author VARCHAR NOT NULL,
  descr TEXT NOT NULL
);

INSERT INTO solutions VALUES
    (12, 'Solution 1', 'Norbert Braun', 'This is the first solution in the database');

INSERT INTO solutions VALUES
    (23, 'Solution 2', 'Norbert Braun', 'This is the second solution in the database');

INSERT INTO solutions VALUES
    (17, 'Solution 1', 'Norbert Braun', 'This is the first solution in the database');

INSERT INTO solutions VALUES
    (7, 'Solution 2', 'Norbert Braun', 'This is the second solution in the database');

INSERT INTO solutions VALUES
    (89, 'Solution 1', 'Norbert Braun', 'This is the first solution in the database');

INSERT INTO solutions VALUES
    (34, 'Solution 2', 'Norbert Braun', 'This is the second solution in the database');