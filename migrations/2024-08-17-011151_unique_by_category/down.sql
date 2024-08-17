ALTER TABLE "listings"
  DROP CONSTRAINT "category_pkey",
  ADD COLUMN "id" SERIAL PRIMARY KEY;

CREATE INDEX "listings_category_index" ON "listings" ("category");
