CREATE TABLE IF NOT EXISTS countries (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS cities (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    country_id INT NOT NULL REFERENCES countries (id)
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(50) NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    city_id INT REFERENCES cities (id) NOT NULL,
    profile_picture_url varchar(100),
    description VARCHAR(2000) NOT NULL,
    birth_year INT NOT NULL
);

CREATE TABLE IF NOT EXISTS bands (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    profile_picture BYTEA,
    description VARCHAR(2000) NOT NULL,
    city_id INT REFERENCES cities (id) NOT NULL
);

CREATE TABLE IF NOT EXISTS instruments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS genres (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_plays_genre (
    user_id INT REFERENCES users (id),
    genre_id INT REFERENCES genres (id),
    PRIMARY KEY (user_id, genre_id)
);

CREATE TABLE IF NOT EXISTS band_plays_genre (
    band_id INT REFERENCES bands (id),
    genre_id INT REFERENCES genres (id),
    PRIMARY KEY (band_id, genre_id)
);

CREATE TABLE IF NOT EXISTS user_plays_instrument (
    user_id INT REFERENCES users (id) NOT NULL,
    instrument_id INT REFERENCES instruments (id) NOT NULL,
    playing_years INT NOT NULL,
    skill_level INT NOT NULL,
    PRIMARY KEY (user_id, instrument_id)
);

CREATE TABLE IF NOT EXISTS open_positions (
    band_id INT REFERENCES bands (id),
    instrument_id INT REFERENCES instruments (id),
    PRIMARY KEY (band_id, instrument_id)
);

CREATE TABLE IF NOT EXISTS memberships (
    user_id INT REFERENCES users (id),
    band_id INT REFERENCES bands (id),
    instrument_id INT REFERENCES instruments (id),
    PRIMARY KEY (user_id, band_id)
);

CREATE TABLE IF NOT EXISTS band_applications (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users (id),
    band_id INT REFERENCES bands (id),
    instrument_id INT REFERENCES instruments (id)
);

CREATE TABLE IF NOT EXISTS band_invitations (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users (id),
    band_id INT REFERENCES bands (id),
    instrument_id INT REFERENCES instruments (id)
);

CREATE TABLE IF NOT EXISTS practice_sessions (
    id SERIAL PRIMARY KEY,
    band_id INT REFERENCES bands (id) NOT NULL,
    place VARCHAR(100) NOT NULL,
    date_time TIMESTAMP NOT NULL,
    note VARCHAR(2000) NOT NULL
);

CREATE TABLE IF NOT EXISTS practice_session_attendants (
    practice_session_id INT REFERENCES practice_sessions (id) NOT NULL,
    user_id INT REFERENCES users (id) NOT NULL,
    PRIMARY KEY (practice_session_id, user_id)
);

CREATE TABLE IF NOT EXISTS jams (
    id SERIAL PRIMARY KEY,
    organizer_id INT REFERENCES users (id) NOT NULL,
    place VARCHAR(100) NOT NULL,
    date_time TIMESTAMP NOT NULL,
    note VARCHAR(2000) NOT NULL
);

CREATE TABLE IF NOT EXISTS jam_invitations (
    jam_id INT REFERENCES jams (id) NOT NULL,
    user_id INT REFERENCES users (id) NOT NULL,
    PRIMARY KEY (jam_id, user_id)
);
