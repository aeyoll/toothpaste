CREATE TABLE IF NOT EXISTS paste (
    `id`          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `filename`    TEXT NOT NULL,
    `content`     TEXT NOT NULL,
    `create_time` DATETIME NOT NULL
);