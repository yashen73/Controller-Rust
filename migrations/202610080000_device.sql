CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE device(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nominatedid VARCHAR(50),
    userid UUID NOT NULL,
    userusername VARCHAR(50) NOT NULL,
    category VARCHAR(50) NOT NULL,
    model VARCHAR(50) NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    hostname VARCHAR(50) NOT NULL,
    port_no BIGINT NOT NULL,
    ip_addr INET,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW() 
);