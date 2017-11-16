\c "file"
ALTER DATABASE article SET TIMEZONE TO 'UTC';
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE "user" (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id       UUID UNIQUE,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp
);

CREATE TABLE file_type (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  mime              VARCHAR(255) UNIQUE,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp
);

CREATE TABLE file (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id       UUID UNIQUE DEFAULT gen_random_uuid() NOT NULL,
  active            BOOL DEFAULT TRUE                     NOT NULL,
  filename          VARCHAR(255)                          NOT NULL,
  "size"            BIGINT                                NOT NULL,
  file_type_id      INT REFERENCES file_type (id)         NOT NULL,
  creation_datetime TIMESTAMP WITHOUT TIME ZONE DEFAULT localtimestamp,
  creator_id        INT REFERENCES "user" (id)
);


CREATE OR REPLACE FUNCTION get_user_id(external_id_ UUID)
  RETURNS INT AS
$$
INSERT INTO "user" ("external_id")
  SELECT external_id_
  WHERE NOT EXISTS(SELECT id
                   FROM "user"
                   WHERE external_id = external_id_);
-- see how it behaves on heavy concurrency
SELECT id
FROM "user"
WHERE external_id = external_id_;
$$ LANGUAGE SQL STRICT;


CREATE OR REPLACE FUNCTION get_file_type_id(mime_ VARCHAR(255))
  RETURNS INT AS
$$
INSERT INTO "file_type" ("mime")
  SELECT mime_
  WHERE NOT EXISTS(SELECT id
                   FROM "file_type"
                   WHERE mime = mime_);
-- see how it behaves on heavy concurrency
SELECT id
FROM "file_type"
WHERE mime = mime_;
$$ LANGUAGE SQL STRICT;

CREATE OR REPLACE FUNCTION create_file(filename_ VARCHAR(255), size_ BIGINT, mime VARCHAR(255), external_id_ UUID)
  RETURNS UUID AS
$$
INSERT INTO "file" (filename, "size", file_type_id, creator_id)
VALUES (filename_, size_, get_file_type_id(mime), get_user_id(external_id_))
RETURNING "file".external_id;
$$ LANGUAGE SQL STRICT;

CREATE OR REPLACE FUNCTION get_file(external_id_ UUID)
  RETURNS TABLE(
    "size"   BIGINT,
    filename VARCHAR(255),
    mime     VARCHAR(255)
  ) AS
$$
SELECT
  "file".size,
  "file".filename,
  "file_type".mime
FROM "file"
  INNER JOIN "file_type" ON "file".file_type_id = file_type.id
WHERE "file".active AND file.external_id = external_id_
$$ LANGUAGE SQL STRICT STABLE;

CREATE OR REPLACE FUNCTION deactivate_file(external_id_ UUID, creator_ UUID)
  RETURNS BOOL AS
$$
WITH file_of_user AS (
    SELECT "file".id AS file_id
    FROM "file"
    WHERE "file"."external_id" = external_id_
          AND "file".creator_id = get_user_id(creator_)
)
UPDATE "file"
SET active = CASE
             WHEN (SELECT count(*)
                   FROM file_of_user) > 0
               THEN
                 FALSE
             ELSE active
             END
WHERE "file".active AND "file".external_id = external_id_
RETURNING (SELECT count(*)
                   FROM file_of_user) > 0;
$$ LANGUAGE SQL STRICT VOLATILE;