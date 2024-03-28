CREATE TABLE "listings" (
  "id"        SERIAL PRIMARY KEY,
  "category"  VARCHAR NOT NULL,
  "url"       VARCHAR NOT NULL,
  "timestamp" TIMESTAMP DEFAULT current_timestamp NOT NULL,
  CONSTRAINT "unique_url" UNIQUE ("url")
);

CREATE INDEX "listings_category_index" ON "listings" ("category");
