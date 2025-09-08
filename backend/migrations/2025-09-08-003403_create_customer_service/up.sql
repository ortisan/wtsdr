CREATE TABLE IF NOT EXISTS customer_services
(
    id          VARCHAR(36) PRIMARY KEY,
    user_id     VARCHAR(36)   NOT NULL,
    name        VARCHAR(120)  NOT NULL,
    description VARCHAR(2048) NOT NULL,
    latitude    DOUBLE PRECISION NOT NULL,
    longitude   DOUBLE PRECISION NOT NULL,
    phone       VARCHAR(16)   NOT NULL, -- E.164 up to 15 digits plus '+'
    website     VARCHAR(2048) NULL,
    deleted     BOOLEAN       NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ   NOT NULL,
    updated_at  TIMESTAMPTZ   NOT NULL,
    deleted_at  TIMESTAMPTZ   NULL,
    CONSTRAINT fk_customer_services_user
        FOREIGN KEY (user_id) REFERENCES users (id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_customer_services_deleted ON customer_services (deleted);
CREATE INDEX IF NOT EXISTS idx_customer_services_user_id ON customer_services (user_id);

CREATE TABLE IF NOT EXISTS customer_service_photos
(
    id         BIGSERIAL PRIMARY KEY,
    service_id VARCHAR(36)   NOT NULL,
    url        VARCHAR(2048) NOT NULL,
    title      VARCHAR(255)  NULL,
    position   INTEGER       NOT NULL DEFAULT 0,
    CONSTRAINT fk_customer_services_photos_service
        FOREIGN KEY (service_id) REFERENCES customer_services (id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_customer_services_photos_service ON customer_service_photos (service_id);
CREATE INDEX IF NOT EXISTS idx_customer_services_photos_service_position ON customer_service_photos (service_id, position);

CREATE TABLE IF NOT EXISTS customer_service_tags
(
    service_id VARCHAR(36)  NOT NULL,
    key        VARCHAR(120) NOT NULL,
    value      VARCHAR(255) NOT NULL,
    CONSTRAINT pk_customer_services_tags PRIMARY KEY (service_id, key),
    CONSTRAINT fk_customer_services_tags_service
        FOREIGN KEY (service_id) REFERENCES customer_services (id)
        ON DELETE CASCADE
);

