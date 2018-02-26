-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  full_name VARCHAR NOT NULL,
  email TEXT NOT NULL,
  pass VARCHAR NOT NULL
);

INSERT INTO users VALUES
    (12, 'Braun Norbert', 'bnorbertjs@gmail.com', '$2y$12$o4Z4WpfDtWRanb.eSlVIuOMHFviNg4XEIhY7S5EgwJvH.IIbtA6W2');