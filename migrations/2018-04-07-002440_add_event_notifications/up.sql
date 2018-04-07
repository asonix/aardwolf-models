-- Your SQL goes here
CREATE TABLE event_notifications (
    id SERIAL PRIMARY KEY,
    event_id INTEGER REFERENCES events(id) ON DELETE CASCADE NOT NULL,
    timer_id INTEGER REFERENCES timers(id) ON DELETE CASCADE NOT NULL
);
