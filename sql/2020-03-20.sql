CREATE TABLE IF NOT EXISTS users (
    id SERIAL NOT NULL,
    uid VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    icon TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE INDEX ON users (uid);

CREATE TABLE IF NOT EXISTS communities (
    id SERIAL NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    public BOOLEAN DEFAULT TRUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE INDEX ON communities (name);

CREATE TABLE IF NOT EXISTS restaurants (
    id SERIAL NOT NULL,
    place_id VARCHAR(255) NOT NULL,
    name VARCHAR(1024) NOT NULL,
    lat NUMERIC NOT NULL,
    lng NUMERIC NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE INDEX ON restaurants (name, lat, lng, place_id);

CREATE TABLE IF NOT EXISTS members (
    id SERIAL NOT NULL,
    user_id INTEGER NOT NULL,
    community_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (community_id) REFERENCES communities(id)
);

CREATE TABLE IF NOT EXISTS pins (
    id SERIAL NOT NULL,
    restaurant_id INTEGER NOT NULL,
    community_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (restaurant_id) REFERENCES restaurants(id),
    FOREIGN KEY (community_id) REFERENCES communities(id)
);

CREATE TABLE IF NOT EXISTS reviews (
    id SERIAL NOT NULL,
    pin_id INTEGER NOT NULL,
    member_id INTEGER NOT NULL,
    rate INTEGER,
    comment TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (member_id) REFERENCES members(id),
    FOREIGN KEY (pin_id) REFERENCES pins(id)
);