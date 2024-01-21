CREATE TABLE IF NOT EXISTS logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    aircraft_type TEXT NOT NULL,
    aircraft_registration TEXT NOT NULL,
    captain TEXT NOT NULL,
    holders_capacity TEXT NOT NULL,
    from_location TEXT NOT NULL,
    to_location TEXT NOT NULL,
    duration TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
