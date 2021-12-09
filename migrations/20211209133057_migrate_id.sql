ALTER TABLE paste RENAME TO _paste_old;

CREATE TABLE paste (
  `id`           TEXT PRIMARY KEY NOT NULL,
  `filename`     TEXT NOT NULL,
  `content`      TEXT NOT NULL,
  `create_time`  DATETIME NOT NULL,
  `expire_after` INTEGER DEFAULT 0,
  `expire_time`  DATETIME NULL
);

INSERT INTO paste(id, filename, content, create_time, expire_after, expire_time)
SELECT id, filename, content, create_time, expire_after, expire_time
FROM _paste_old;

DROP TABLE _paste_old;
