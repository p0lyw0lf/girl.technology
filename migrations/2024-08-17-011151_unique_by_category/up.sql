DROP INDEX "listings_category_index";

ALTER TABLE "listings"
  DROP COLUMN "id",
  ADD CONSTRAINT "category_pkey" PRIMARY KEY ("category");
