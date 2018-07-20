CREATE OR REPLACE FUNCTION update_timestamp() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated = now(); 
    RETURN NEW;
END;
$$ language 'plpgsql';

create table if not exists users (
    id serial primary key,
    name text,
    email text not null unique,
    password text not null,
    avatar text,
    is_verified bool not null default false,
    has_verified_email bool not null default false,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

CREATE TRIGGER user_updated BEFORE INSERT OR UPDATE ON users
FOR EACH ROW EXECUTE PROCEDURE update_timestamp();
