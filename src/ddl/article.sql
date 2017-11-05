CREATE TABLE "user" (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id       UUID UNIQUE,
  creation_datetime TIMESTAMP DEFAULT current_timestamp
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

CREATE TABLE "article" (
  id                   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id          UUID UNIQUE DEFAULT gen_random_uuid() NOT NULL,
  creation_datetime    TIMESTAMP DEFAULT current_timestamp,
  active               BOOL DEFAULT FALSE                    NOT NULL,
  publication_datetime TIMESTAMP                             NOT NULL
);

CREATE INDEX ON article (active, publication_datetime DESC);

CREATE TABLE "article_edition" (
  id                   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id          UUID UNIQUE DEFAULT gen_random_uuid() NOT NULL,
  article_id           INT REFERENCES article (id)           NOT NULL,
  title                TEXT,
  content              TEXT                                  NOT NULL,
  creation_datetime    TIMESTAMP DEFAULT current_timestamp,
  publication_datetime TIMESTAMP                             NOT NULL,
  UNIQUE (article_id, id)
);

CREATE TABLE "article_current_edition" (
  article_id         INT REFERENCES article (id) PRIMARY KEY,
  article_edition_id INT NOT NULL,
  FOREIGN KEY (article_edition_id, article_id) REFERENCES article_edition (id, article_id)
);

CREATE TABLE "tag" (
  id                INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id       UUID UNIQUE DEFAULT gen_random_uuid() NOT NULL,
  name              VARCHAR(63) UNIQUE                    NOT NULL,
  creation_datetime TIMESTAMP DEFAULT current_timestamp
);

CREATE TABLE "article_tag" (
  article_id INT REFERENCES article (id),
  tag_id     INT REFERENCES tag (id),
  PRIMARY KEY (article_id, tag_id)
);


CREATE TABLE "comment" (
  id                   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id          UUID UNIQUE DEFAULT gen_random_uuid() NOT NULL,
  active               BOOL DEFAULT FALSE                    NOT NULL,
  user_id              INT REFERENCES "user" ("id")          NOT NULL,
  article_id           INT REFERENCES "article" ("id")       NOT NULL,
  creation_datetime    TIMESTAMP DEFAULT current_timestamp,
  publication_datetime TIMESTAMP                             NOT NULL
);

CREATE INDEX ON comment ("active", "article_id", "publication_datetime" DESC);

CREATE TABLE "comment_edition" (
  id                   INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  external_id          UUID UNIQUE DEFAULT gen_random_uuid() NOT NULL,
  content              TEXT                                  NOT NULL,
  comment_id           INT REFERENCES comment (id),
  publication_datetime TIMESTAMP                             NOT NULL,
  creation_datetime    TIMESTAMP DEFAULT current_timestamp,
  UNIQUE (comment_id, id)
);

CREATE TABLE "comment_current_edition" (
  comment_id         INT REFERENCES comment (id) PRIMARY KEY,
  comment_edition_id INT NOT NULL,
  FOREIGN KEY (comment_edition_id, comment_id) REFERENCES comment_edition (id, comment_id)
);

CREATE OR REPLACE FUNCTION get_article_list(fetch_length INT, before_external_id UUID)
  RETURNS
    TABLE(
      id                   UUID,
      title                TEXT,
      publication_datetime TIMESTAMP,
      edition_datetime     TIMESTAMP) AS
$$
SELECT
  article.external_id,
  edition.title,
  article.publication_datetime,
  edition.publication_datetime
FROM article
  INNER JOIN article_current_edition a2e ON article.id = a2e.article_id
  INNER JOIN article_edition edition ON edition.id = a2e.article_edition_id
WHERE article.active
      AND (before_external_id IS NULL
           OR article.publication_datetime < (SELECT publication_datetime
                                              FROM article a
                                              WHERE a.external_id = before_external_id)) -- index here
ORDER BY article.publication_datetime DESC
LIMIT fetch_length
$$ LANGUAGE SQL STRICT IMMUTABLE;


CREATE OR REPLACE FUNCTION  get_article_list_by_tag(tag_id_ UUID, fetch_length INT, before_external_id UUID)
  RETURNS
    TABLE(
      id                   UUID,
      title                TEXT,
      publication_datetime TIMESTAMP,
      edition_datetime     TIMESTAMP) AS
$$
SELECT
  article.external_id,
  edition.title,
  article.publication_datetime,
  edition.publication_datetime
FROM article
  INNER JOIN article_current_edition current_edition ON article.id = current_edition.article_id
  INNER JOIN article_edition edition ON edition.id = current_edition.article_edition_id
  INNER JOIN article_tag ON article_tag.article_id = article.id
  INNER JOIN "tag" ON article_tag.tag_id = "tag".id AND "tag".external_id = tag_id_
WHERE article.active
      AND (before_external_id IS NULL
           OR article.publication_datetime < (SELECT publication_datetime
                                              FROM article a
                                              WHERE a.external_id = before_external_id)) -- index here
ORDER BY article.publication_datetime DESC
LIMIT fetch_length
$$ LANGUAGE SQL STRICT IMMUTABLE;



CREATE OR REPLACE FUNCTION get_article(external_id_ UUID)
  RETURNS
    TABLE(
      id                   UUID,
      title                TEXT,
      publication_datetime TIMESTAMP,
      edition_datetime     TIMESTAMP) AS
$$
SELECT
  article.external_id,
  edition.title,
  article.publication_datetime,
  edition.publication_datetime
FROM article
  INNER JOIN article_current_edition a2e ON article.id = a2e.article_id
  INNER JOIN article_edition edition ON edition.id = a2e.article_edition_id
WHERE article.external_id = external_id_ AND article.active
$$ LANGUAGE SQL STRICT IMMUTABLE;


CREATE OR REPLACE FUNCTION get_article_content(article_id_ UUID)
  RETURNS TEXT AS
$$
SELECT edition.content
FROM article
  INNER JOIN article_current_edition current_edition ON article.id = current_edition.article_id
  INNER JOIN article_edition edition ON edition.id = current_edition.article_edition_id
WHERE article.active AND
      article.external_id = article_id_
$$ LANGUAGE SQL STRICT IMMUTABLE;

CREATE OR REPLACE FUNCTION get_article_comment_list(article_id_ UUID, before_external_id UUID, fetch_length INT)
  RETURNS TABLE(
    id                   UUID,
    user_id              UUID,
    publication_datetime TIMESTAMP,
    edition_datetime     TIMESTAMP
  ) AS $$
SELECT
  "comment".external_id,
  "user".external_id,
  "comment".publication_datetime,
  "edition".publication_datetime
FROM "comment"
  INNER JOIN "comment_current_edition" current_edition ON current_edition.comment_id = "comment".id
  INNER JOIN comment_edition edition ON edition.id = current_edition.comment_edition_id
  INNER JOIN "user" ON "user".id = "comment".user_id
WHERE "comment".active
      AND "comment".article_id = (SELECT id
                                  FROM article
                                  WHERE article.external_id = article_id_)
      AND (before_external_id IS NULL
           OR "comment".publication_datetime < (SELECT publication_datetime
                                                FROM "comment" a
                                                WHERE a.external_id = before_external_id)) -- index here
ORDER BY "comment".publication_datetime
LIMIT fetch_length
$$ LANGUAGE SQL STRICT IMMUTABLE;


CREATE OR REPLACE FUNCTION get_comment_content(comment_id_ UUID)
  RETURNS TEXT AS $$
SELECT "edition".content
FROM "comment"
  INNER JOIN "comment_current_edition" current_edition ON current_edition.comment_id = "comment".id
  INNER JOIN comment_edition edition ON edition.id = current_edition.comment_edition_id
WHERE "comment".external_id = comment_id_ AND "comment".active
$$ LANGUAGE SQL STRICT IMMUTABLE;


CREATE OR REPLACE FUNCTION create_comment(article_id_ UUID, user_id_ UUID, publication_datetime_ TIMESTAMP,
                                          content_    TEXT)
  RETURNS UUID AS $$
DECLARE
  new_comment_id         INT;
  new_comment_edition_id INT;
BEGIN
  INSERT INTO "comment" ("article_id", "user_id", "publication_datetime")
  VALUES (
    (SELECT id
     FROM article
     WHERE article.external_id = article_id_),
    get_user_id(user_id_) ,
    publication_datetime_
  )
  RETURNING id
    INTO new_comment_id;

  INSERT INTO "comment_edition" ("comment_id", "publication_datetime", content)
  VALUES (new_comment_id, publication_datetime_, content_)
  RETURNING id
    INTO new_comment_edition_id;

  INSERT INTO "comment_current_edition" ("comment_id", "comment_edition_id")
  VALUES (new_comment_id, new_comment_edition_id);
  RETURN new_comment_id;
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_article_tag_list(article_id_ UUID)
  RETURNS TABLE(id UUID, name TEXT) AS $$
  SELECT "tag".external_id, "tag".name
  FROM "tag"
    INNER JOIN article_tag ON article_tag.tag_id = "tag".id
    INNER JOIN article ON article.id = article_tag.article_id
  WHERE article.external_id = article_id_;
$$ LANGUAGE SQL STRICT IMMUTABLE;


