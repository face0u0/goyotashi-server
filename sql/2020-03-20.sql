CREATE TABLE IF NOT EXISTS users (
    id SERIAL NOT NULL,
    uid VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE (uid)
);
CREATE INDEX ON users (uid);

CREATE TABLE IF NOT EXISTS communities (
    id SERIAL NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    public BOOLEAN DEFAULT TRUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id)
);
CREATE INDEX ON communities (name);

CREATE TABLE IF NOT EXISTS restaurants (
    id SERIAL NOT NULL,
    vendor INTEGER NOT NULL,
    place_id VARCHAR(255) NOT NULL,
    name VARCHAR(1024) NOT NULL,
    lat DOUBLE PRECISION NOT NULL,
    lng DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE (place_id)
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
    FOREIGN KEY (community_id) REFERENCES communities(id),
    UNIQUE (user_id, community_id)
);

CREATE TABLE IF NOT EXISTS pins (
    id SERIAL NOT NULL,
    restaurant_id INTEGER NOT NULL,
    community_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (restaurant_id) REFERENCES restaurants(id),
    FOREIGN KEY (community_id) REFERENCES communities(id),
    UNIQUE (restaurant_id, community_id)
);

CREATE TABLE IF NOT EXISTS reviews (
    id SERIAL NOT NULL,
    pin_id INTEGER NOT NULL,
    member_id INTEGER NOT NULL,
    comment TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (member_id) REFERENCES members(id),
    FOREIGN KEY (pin_id) REFERENCES pins(id),
    UNIQUE (pin_id, member_id)
);