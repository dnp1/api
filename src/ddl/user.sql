CREATE TABLE "user" (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id       UUID UNIQUE DEFAULT gen_random_uuid() NOT NULL,
  creation_datetime TIMESTAMP DEFAULT current_timestamp
);

CREATE OR REPLACE FUNCTION get_user_id(external_id_ UUID)
  RETURNS INT AS
$$
SELECT id
FROM "user"
WHERE external_id = external_id_;
$$ LANGUAGE SQL STRICT;


CREATE TABLE email (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  address           VARCHAR(255) UNIQUE CHECK (address ~ '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
  creation_datetime TIMESTAMP DEFAULT current_timestamp
);

CREATE INDEX ON email (address);


CREATE TABLE user_email (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  "email_id"        INT REFERENCES email (id)  NOT NULL,
  "user_id"         INT REFERENCES "user" (id) NOT NULL,
  active            BOOLEAN   DEFAULT TRUE,
  creation_datetime TIMESTAMP DEFAULT current_timestamp
);

CREATE INDEX ON user_email (active, email_id);
CREATE UNIQUE INDEX ON user_email (email_id)
  WHERE active;
CREATE INDEX ON user_email (user_id, creation_datetime DESC);

CREATE TABLE user_password (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  user_id           INT REFERENCES "user" (id) NOT NULL,
  password          TEXT                       NOT NULL,
  active            BOOLEAN   DEFAULT TRUE,
  creation_datetime TIMESTAMP DEFAULT current_timestamp
);

CREATE INDEX ON user_password (user_id, creation_datetime DESC);
CREATE INDEX ON user_password ("active", "user_id");
CREATE UNIQUE INDEX ON user_password (user_id)
  WHERE active;

CREATE TABLE user_name (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  user_id           INT REFERENCES "user" (id) NOT NULL,
  family_name       TEXT                       NOT NULL,
  given_name        TEXT                       NOT NULL,
  creation_datetime TIMESTAMP DEFAULT current_timestamp
);

CREATE INDEX ON user_name (user_id, creation_datetime DESC);

CREATE TABLE user_avatar (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  user_id           INT REFERENCES "user" (id) NOT NULL,
  file_id           UUID                       NOT NULL,
  creation_datetime TIMESTAMP DEFAULT current_timestamp,
  UNIQUE (user_id, id)
);

CREATE INDEX ON user_avatar (user_id, creation_datetime DESC);


CREATE TABLE session (
  id                BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  creation_datetime TIMESTAMP DEFAULT current_timestamp
);

CREATE TABLE authentication_attempt (
  id                BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  session_id        BIGINT REFERENCES session (id) DEFAULT NULL,
  user_id           INT REFERENCES "user" (id)     DEFAULT NULL,
  creation_datetime TIMESTAMP                      DEFAULT current_timestamp
);


CREATE OR REPLACE FUNCTION authenticate(session_id_ BIGINT, email_ TEXT, password_ TEXT)
  RETURNS BOOLEAN AS
$$
WITH res AS (
    SELECT
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
INSERT INTO authentication_attempt (session_id, user_id)
  SELECT *
  FROM (SELECT *
        FROM res
        UNION ALL SELECT
                    NULL,
                    NULL) AS r
  LIMIT 1
RETURNING user_id IS DISTINCT FROM NULL;
$$ LANGUAGE SQL STRICT;