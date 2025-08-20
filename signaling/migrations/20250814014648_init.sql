-- Только комнаты и участники
CREATE TABLE rooms (
    id SERIAL PRIMARY KEY,
    public_code VARCHAR(255) NOT NULL UNIQUE, 
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    is_active BOOLEAN DEFAULT true NOT NULL
);

CREATE TABLE participants (
    id SERIAL PRIMARY KEY,
    room_id INTEGER REFERENCES rooms(id) ON DELETE CASCADE NOT NULL,
    client_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    is_host BOOLEAN DEFAULT false NOT NULL,
    UNIQUE(room_id, client_id)
);


-- Таблица сообщений чата
CREATE TABLE chat_messages (
    id SERIAL PRIMARY KEY,
    room_id INTEGER NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    sender_id INTEGER NOT NULL REFERENCES participants(id) ON DELETE CASCADE,
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);
