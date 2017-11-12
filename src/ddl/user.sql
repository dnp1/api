\c "user"
ALTER DATABASE article SET timezone TO 'UTC';
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE "user" (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id       UUID UNIQUE DEFAULT gen_random_uuid() NOT NULL,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp
);


CREATE TABLE email (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  address           VARCHAR(255) UNIQUE CHECK (address ~ '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp
);


CREATE TABLE user_email (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  "email_id"        INT REFERENCES email (id)  NOT NULL,
  "user_id"         INT REFERENCES "user" (id) NOT NULL,
  active            BOOLEAN   DEFAULT TRUE,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp
);

CREATE UNIQUE INDEX ON user_email (email_id)
  WHERE active;
CREATE UNIQUE INDEX ON user_email (user_id)
  WHERE active;

CREATE TABLE user_password (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  user_id           INT REFERENCES "user" (id) NOT NULL,
  password          TEXT                       NOT NULL,
  active            BOOLEAN   DEFAULT TRUE,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp
);

CREATE UNIQUE INDEX ON user_password (user_id)
  WHERE active;

CREATE TABLE user_name (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  user_id           INT REFERENCES "user" (id) NOT NULL,
  family_name       TEXT                       NOT NULL,
  given_name        TEXT                       NOT NULL,
  active            BOOLEAN   DEFAULT TRUE,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp
);

CREATE UNIQUE INDEX ON user_name (user_id)
  WHERE active;

CREATE TABLE user_avatar (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  user_id           INT REFERENCES "user" (id) NOT NULL,
  file_id           UUID                       NOT NULL,
  active            BOOLEAN   DEFAULT TRUE,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp
);

CREATE UNIQUE INDEX ON user_avatar (user_id)
  WHERE active;


CREATE TABLE session (
  id                BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp
);

CREATE TABLE authentication_attempt (
  id                BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  session_id        BIGINT REFERENCES session (id) DEFAULT NULL,
  user_id           INT REFERENCES "user" (id)     DEFAULT NULL,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE                      DEFAULT localtimestamp,
  host_address      CIDR NOT NULL
);

CREATE TABLE password_recovery (
  id                BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  session_id        BIGINT REFERENCES session (id) DEFAULT NULL,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE                      DEFAULT localtimestamp
);

CREATE OR REPLACE FUNCTION get_user_id(external_id_ UUID)
  RETURNS INT AS
$$
SELECT id
FROM "user"
WHERE external_id = external_id_;
$$ LANGUAGE SQL STRICT;


CREATE OR REPLACE FUNCTION get_email_id(email_ VARCHAR(255))
  RETURNS INT AS
$$
INSERT INTO "email" ("address")
  SELECT email_
  WHERE NOT EXISTS(SELECT id
                   FROM "email"
                   WHERE address = email_);
-- see how it behaves on heavy concurrency
SELECT id
FROM "email"
WHERE address = email_;
$$ LANGUAGE SQL STRICT;


CREATE OR REPLACE FUNCTION authenticate(session_id_ BIGINT, email_ TEXT, password_ TEXT, cidr_ CIDR)
  RETURNS UUID AS
$$
WITH res AS (
    SELECT
      cidr_,
      session_id_,
      user_password.user_id
    FROM "user_password"
      INNER JOIN "session" ON "session".id = session_id_
      INNER JOIN user_email ON user_email.user_id = user_password.user_id
      INNER JOIN email ON email.id = user_email.email_id
    WHERE "user_password".active
          AND user_email.active
          AND email.address = email_
          AND user_password."password" = crypt(password_, user_password.password)
)
INSERT INTO authentication_attempt (host_address, session_id, user_id)
  SELECT *
  FROM (SELECT *
        FROM res
        UNION ALL SELECT
                    cidr_,
                    (SELECT id
                     FROM session
                     WHERE id = session_id_),
                    NULL) AS r
  LIMIT 1
RETURNING (SELECT "user".external_id FROM "user" WHERE "user".id = user_id);
$$ LANGUAGE SQL STRICT;

CREATE OR REPLACE FUNCTION create_user(session_id_  BIGINT, email_ TEXT, password_ TEXT, given_name_ TEXT,
                                       family_name_ TEXT)
  RETURNS BOOLEAN AS
$$
DECLARE
  user_id_           BIGINT;
  session_not_exists BOOL;
  email_id_          INT;
BEGIN
  SELECT NOT EXISTS(SELECT id
                    FROM session
                    WHERE id = session_id_)
  INTO session_not_exists;
  IF session_not_exists
  THEN
    RAISE EXCEPTION 'INVALID_SESSION %d', session_id_;
    RETURN FALSE;
  END IF;
  INSERT INTO "user" DEFAULT VALUES RETURNING id INTO user_id_;

  INSERT INTO "user_password" ("user_id", "password") VALUES (user_id_, crypt(password_, gen_salt('bf', 12)));
  SELECT get_email_id(email_)
  INTO email_id_;
  INSERT INTO "user_email" ("user_id", "email_id") VALUES (user_id_, email_id_);
  INSERT INTO "user_name" ("user_id", "family_name", "given_name") VALUES (user_id_, family_name_, given_name_);
  RETURN TRUE;
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_user_name(user_id_ UUID)
  RETURNS TABLE(given_name TEXT, family_name TEXT) AS
$$
SELECT
  user_name.family_name,
  user_name.given_name
FROM user_name

WHERE active AND user_name.user_id = get_user_id(user_id_)
$$ LANGUAGE SQL STRICT STABLE;

CREATE OR REPLACE FUNCTION get_user_avatar(user_id_ UUID)
  RETURNS UUID AS
$$
SELECT
  user_avatar.file_id
FROM user_avatar
WHERE active AND user_avatar.user_id = get_user_id(user_id_)
$$ LANGUAGE SQL STRICT STABLE;


CREATE OR REPLACE FUNCTION get_user_email(user_id_ UUID)
  RETURNS TEXT AS
$$
SELECT
  email.address
FROM email
  INNER JOIN user_email ON email.id = user_email.email_id
WHERE active AND user_email.user_id = get_user_id(user_id_)
$$ LANGUAGE SQL STRICT IMMUTABLE ;

CREATE OR REPLACE FUNCTION create_session() RETURNS BIGINT AS
$$
INSERT INTO "session" DEFAULT VALUES RETURNING id;
$$ LANGUAGE SQL

