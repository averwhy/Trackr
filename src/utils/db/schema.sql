CREATE TABLE users (
    id INT PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE user_stats (
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    checked_count INT DEFAULT 0,
    tracked_count INT DEFAULT 0,
    last_checked TIMESTAMP,
    last_tracked TIMESTAMP,
    last_alert TIMESTAMP
);
CREATE TABLE command_stats (
    id SERIAL PRIMARY KEY,
    command_name VARCHAR(100) NOT NULL,
    command_count INT DEFAULT 0,
    last_run TIMESTAMP
);
CREATE TABLE active_trackings (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    agency_id INT NOT NULL,
    line_id VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE agencies (
    id SERIAL PRIMARY KEY,
    short_name VARCHAR(50) NOT NULL,
    long_name VARCHAR(100) NOT NULL,
    api_url TEXT NOT NULL,
    key_env_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Index for ez short_name lookups
CREATE INDEX idx_agencies_short_name ON agencies (short_name);
CREATE TABLE agency_lines (
    id SERIAL PRIMARY KEY,
    agency_id INT NOT NULL REFERENCES agencies(id) ON DELETE CASCADE,
    line_id VARCHAR(50) NOT NULL,
    line_name VARCHAR(100) NOT NULL,
    line_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Index for ez agency_id/line_id lookups
CREATE INDEX idx_agency_lines_agency_id_line_id ON agency_lines (agency_id, line_id);
CREATE TABLE endpoints (
    id SERIAL PRIMARY KEY,
    agency_id INT NOT NULL REFERENCES agencies(id) ON DELETE CASCADE,
    endpoint_type VARCHAR(50) NOT NULL,
    endpoint_url TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Index for faster lookups by agency_id and endpoint_type
CREATE INDEX idx_endpoints_agency_id_endpoint_type ON endpoints (agency_id, endpoint_type);
CREATE TABLE endpoint_pointers (
    id SERIAL PRIMARY KEY,
    endpoint_id INT NOT NULL REFERENCES endpoints(id) ON DELETE CASCADE,
    pointer_key VARCHAR(100) NOT NULL,
    pointer_path TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_endpoint_pointers_endpoint_id_pointer_key ON endpoint_pointers (endpoint_id, pointer_key);
CREATE TABLE statistics (
    id SERIAL PRIMARY KEY,
    agency_id INT REFERENCES agencies(id) ON DELETE CASCADE,
    line_id INT REFERENCES agency_lines(id) ON DELETE CASCADE,
    checked_count INT DEFAULT 0,
    tracked_count INT DEFAULT 0,
    alert_count INT DEFAULT 0,
    last_checked TIMESTAMP,
    last_tracked TIMESTAMP,
    last_alert TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Index for faster lookups by agency_id and line_id
CREATE INDEX idx_statistics_agency_id ON statistics (agency_id);
CREATE INDEX idx_statistics_line_id ON statistics (line_id);