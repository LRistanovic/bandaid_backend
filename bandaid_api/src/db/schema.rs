// @generated automatically by Diesel CLI.

diesel::table! {
    band_applications (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        band_id -> Nullable<Int4>,
        instrument_id -> Nullable<Int4>,
    }
}

diesel::table! {
    band_invitations (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        band_id -> Nullable<Int4>,
        instrument_id -> Nullable<Int4>,
    }
}

diesel::table! {
    band_plays_genre (band_id, genre_id) {
        band_id -> Int4,
        genre_id -> Int4,
    }
}

diesel::table! {
    bands (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        profile_picture -> Nullable<Bytea>,
        #[max_length = 2000]
        description -> Varchar,
        city_id -> Int4,
    }
}

diesel::table! {
    cities (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        country_id -> Int4,
    }
}

diesel::table! {
    countries (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    genres (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    instruments (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    jam_invitations (jam_id, user_id) {
        jam_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    jams (id) {
        id -> Int4,
        organizer_id -> Int4,
        #[max_length = 100]
        place -> Varchar,
        date_time -> Timestamp,
        #[max_length = 2000]
        note -> Varchar,
    }
}

diesel::table! {
    memberships (user_id, band_id) {
        user_id -> Int4,
        band_id -> Int4,
        instrument_id -> Nullable<Int4>,
    }
}

diesel::table! {
    open_positions (band_id, instrument_id) {
        band_id -> Int4,
        instrument_id -> Int4,
    }
}

diesel::table! {
    practice_session_attendants (practice_session_id, user_id) {
        practice_session_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    practice_sessions (id) {
        id -> Int4,
        band_id -> Int4,
        #[max_length = 100]
        place -> Varchar,
        date_time -> Timestamp,
        #[max_length = 2000]
        note -> Varchar,
    }
}

diesel::table! {
    user_plays_genre (user_id, genre_id) {
        user_id -> Int4,
        genre_id -> Int4,
    }
}

diesel::table! {
    user_plays_instrument (user_id, instrument_id) {
        user_id -> Int4,
        instrument_id -> Int4,
        playing_years -> Int4,
        skill_level -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 60]
        password -> Varchar,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        city_id -> Int4,
        #[max_length = 100]
        profile_picture_url -> Nullable<Varchar>,
        #[max_length = 2000]
        description -> Varchar,
        birth_year -> Int4,
    }
}

diesel::joinable!(band_applications -> bands (band_id));
diesel::joinable!(band_applications -> instruments (instrument_id));
diesel::joinable!(band_applications -> users (user_id));
diesel::joinable!(band_invitations -> bands (band_id));
diesel::joinable!(band_invitations -> instruments (instrument_id));
diesel::joinable!(band_invitations -> users (user_id));
diesel::joinable!(band_plays_genre -> bands (band_id));
diesel::joinable!(band_plays_genre -> genres (genre_id));
diesel::joinable!(bands -> cities (city_id));
diesel::joinable!(cities -> countries (country_id));
diesel::joinable!(jam_invitations -> jams (jam_id));
diesel::joinable!(jam_invitations -> users (user_id));
diesel::joinable!(jams -> users (organizer_id));
diesel::joinable!(memberships -> bands (band_id));
diesel::joinable!(memberships -> instruments (instrument_id));
diesel::joinable!(memberships -> users (user_id));
diesel::joinable!(open_positions -> bands (band_id));
diesel::joinable!(open_positions -> instruments (instrument_id));
diesel::joinable!(practice_session_attendants -> practice_sessions (practice_session_id));
diesel::joinable!(practice_session_attendants -> users (user_id));
diesel::joinable!(practice_sessions -> bands (band_id));
diesel::joinable!(user_plays_genre -> genres (genre_id));
diesel::joinable!(user_plays_genre -> users (user_id));
diesel::joinable!(user_plays_instrument -> instruments (instrument_id));
diesel::joinable!(user_plays_instrument -> users (user_id));
diesel::joinable!(users -> cities (city_id));

diesel::allow_tables_to_appear_in_same_query!(
    band_applications,
    band_invitations,
    band_plays_genre,
    bands,
    cities,
    countries,
    genres,
    instruments,
    jam_invitations,
    jams,
    memberships,
    open_positions,
    practice_session_attendants,
    practice_sessions,
    user_plays_genre,
    user_plays_instrument,
    users,
);
