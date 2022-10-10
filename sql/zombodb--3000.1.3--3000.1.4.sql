DROP FUNCTION IF EXISTS zdb.schema_version() CASCADE;
-- it's imperative for `update-versions.sh` that this function be formatted exactly this way
CREATE FUNCTION zdb.schema_version() RETURNS text LANGUAGE sql AS $$
SELECT '3000.1.4 (d005ed52ba1f2bb159f61fe4f7ebeb0297c03f38)'
$$;
-- src/highlighting/es_highlighting.rs:163
-- zombodb::highlighting::es_highlighting::highlight
-- requires:
--   highlighting::es_highlighting::highlight
CREATE FUNCTION zdb."highlight"(
	"ctid" tid, /* pgx_pg_sys::pg14::ItemPointerData */
	"_highlight_definition" json DEFAULT zdb.highlight() /* pgx::datum::json::Json */
) RETURNS json /* pgx::datum::json::Json */
PARALLEL SAFE IMMUTABLE   STRICT
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'highlight_all_fields_wrapper';

