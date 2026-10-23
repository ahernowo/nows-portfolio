-- Articles being edited in the CMS. Published versions live in git.
CREATE TABLE articles (
  id             TEXT PRIMARY KEY,                 -- 24 hex characters
  slug           TEXT NOT NULL UNIQUE,
  title          TEXT NOT NULL,
  description    TEXT NOT NULL DEFAULT '',
  tags           TEXT NOT NULL DEFAULT '[]',       -- JSON array of strings
  body           TEXT NOT NULL DEFAULT '',         -- Markdown
  date           TEXT,                             -- YYYY-MM-DD chosen by the author, or NULL
  status         TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published')),
  version        INTEGER NOT NULL DEFAULT 1,       -- bumped on every save, for conflict checks
  created_at     INTEGER NOT NULL,                 -- Unix seconds
  updated_at     INTEGER NOT NULL,
  published_at   INTEGER,                          -- first publish
  published_slug TEXT,                             -- address in git; fixed once set
  published_sha  TEXT,                             -- blob sha of the last file written to git
  updated_by     TEXT NOT NULL                     -- GitHub login
);

CREATE INDEX articles_updated ON articles (updated_at DESC);
CREATE INDEX articles_published ON articles (status, published_at DESC);

-- Who did what, for your own peace of mind.
CREATE TABLE audit_log (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  at         INTEGER NOT NULL,
  actor      TEXT NOT NULL,
  action     TEXT NOT NULL CHECK (action IN ('create', 'update', 'publish', 'unpublish', 'delete')),
  article_id TEXT,
  detail     TEXT NOT NULL DEFAULT ''
);

CREATE INDEX audit_log_at ON audit_log (at DESC);
