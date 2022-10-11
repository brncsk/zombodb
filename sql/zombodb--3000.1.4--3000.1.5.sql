DROP FUNCTION IF EXISTS zdb.schema_version() CASCADE;
-- it's imperative for `update-versions.sh` that this function be formatted exactly this way
CREATE FUNCTION zdb.schema_version() RETURNS text LANGUAGE sql AS $$
SELECT '3000.1.5 (4a5d1fb37d7ed243ef5a929167aeb8cb72bff36d)'
$$;
-- src/query_dsl/matches.rs:297
-- zombodb::query_dsl::matches::dsl::match_bool_prefix
CREATE FUNCTION dsl."match_bool_prefix"(
	"field" text, /* &str */
	"query" text, /* &str */
	"operator" pg_catalog.Operator DEFAULT NULL, /* core::option::Option<zombodb::query_dsl::matches::pg_catalog::Operator> */
	"analyzer" text DEFAULT NULL, /* core::option::Option<&str> */
	"minimum_should_match" integer DEFAULT NULL, /* core::option::Option<i32> */
	"fuzziness" integer DEFAULT NULL, /* core::option::Option<i32> */
	"prefix_length" integer DEFAULT NULL, /* core::option::Option<i32> */
	"max_expansions" integer DEFAULT NULL, /* core::option::Option<i32> */
	"fuzzy_transpositions" bool DEFAULT NULL, /* core::option::Option<bool> */
	"fuzzy_rewrite" text DEFAULT NULL /* core::option::Option<&str> */
) RETURNS pg_catalog.ZDBQuery /* zombodb::zdbquery::pg_catalog::ZDBQuery */
IMMUTABLE PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'match_bool_prefix_wrapper';

