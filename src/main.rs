extern crate htmlescape;
extern crate time;
#[macro_use]
extern crate maplit;

use htmlescape::encode_minimal;
use std::fs;
use time::PreciseTime;

enum Token {
    Unset = 0,
    Word = 1,
    Name = 2,
    Symbol = 3,
    Number = 4,
    String = 5,
    Hex = 6,
    HexString = 7,
    Function = 8,
}

const MYSQL: &str = "select`users`.`UserID`,now(6)
from`users`
join`companies`using(`CompanyID`)
where`users`.`Email`='<phil@redshift.com>'
and`companies`.`NetworkID`=x'1541A488C87419F2'
and`users`.`__Active`<>0 
order by`users`.`__Added`desc
limit 1;";

struct E {}

fn main() {
    let reserved = hashmap!{
    "accessible" => E{},
    "account" => E{},
    "action" => E{},
    "active" => E{},
    "add" => E{},
    "admin" => E{},
    "after" => E{},
    "against" => E{},
    "aggregate" => E{},
    "algorithm" => E{},
    "all" => E{},
    "alter" => E{},
    "always" => E{},
    "analyse" => E{},
    "analyze" => E{},
    "and" => E{},
    "any" => E{},
    "as" => E{},
    "asc" => E{},
    "ascii" => E{},
    "asensitive" => E{},
    "at" => E{},
    "autoextend_size" => E{},
    "auto_increment" => E{},
    "avg" => E{},
    "avg_row_length" => E{},
    "backup" => E{},
    "before" => E{},
    "begin" => E{},
    "between" => E{},
    "bigint" => E{},
    "binary" => E{},
    "binlog" => E{},
    "bit" => E{},
    "blob" => E{},
    "block" => E{},
    "bool" => E{},
    "boolean" => E{},
    "both" => E{},
    "btree" => E{},
    "buckets" => E{},
    "by" => E{},
    "byte" => E{},
    "cache" => E{},
    "call" => E{},
    "cascade" => E{},
    "cascaded" => E{},
    "case" => E{},
    "catalog_name" => E{},
    "chain" => E{},
    "change" => E{},
    "changed" => E{},
    "channel" => E{},
    "char" => E{},
    "character" => E{},
    "charset" => E{},
    "check" => E{},
    "checksum" => E{},
    "cipher" => E{},
    "class_origin" => E{},
    "client" => E{},
    "clone" => E{},
    "close" => E{},
    "coalesce" => E{},
    "code" => E{},
    "collate" => E{},
    "collation" => E{},
    "column" => E{},
    "columns" => E{},
    "column_format" => E{},
    "column_name" => E{},
    "comment" => E{},
    "commit" => E{},
    "committed" => E{},
    "compact" => E{},
    "completion" => E{},
    "component" => E{},
    "compressed" => E{},
    "compression" => E{},
    "concurrent" => E{},
    "condition" => E{},
    "connection" => E{},
    "consistent" => E{},
    "constraint" => E{},
    "constraint_catalog" => E{},
    "constraint_name" => E{},
    "constraint_schema" => E{},
    "contains" => E{},
    "context" => E{},
    "continue" => E{},
    "convert" => E{},
    "cpu" => E{},
    "create" => E{},
    "cross" => E{},
    "cube" => E{},
    "cume_dist" => E{},
    "current" => E{},
    "current_date" => E{},
    "current_time" => E{},
    "current_timestamp" => E{},
    "current_user" => E{},
    "cursor" => E{},
    "cursor_name" => E{},
    "data" => E{},
    "database" => E{},
    "databases" => E{},
    "datafile" => E{},
    "date" => E{},
    "datetime" => E{},
    "day" => E{},
    "day_hour" => E{},
    "day_microsecond" => E{},
    "day_minute" => E{},
    "day_second" => E{},
    "deallocate" => E{},
    "dec" => E{},
    "decimal" => E{},
    "declare" => E{},
    "default" => E{},
    "default_auth" => E{},
    "definer" => E{},
    "definition" => E{},
    "delayed" => E{},
    "delay_key_write" => E{},
    "delete" => E{},
    "dense_rank" => E{},
    "desc" => E{},
    "describe" => E{},
    "description" => E{},
    "des_key_file" => E{},
    "deterministic" => E{},
    "diagnostics" => E{},
    "directory" => E{},
    "disable" => E{},
    "discard" => E{},
    "disk" => E{},
    "distinct" => E{},
    "distinctrow" => E{},
    "div" => E{},
    "do" => E{},
    "double" => E{},
    "drop" => E{},
    "dual" => E{},
    "dumpfile" => E{},
    "duplicate" => E{},
    "dynamic" => E{},
    "each" => E{},
    "else" => E{},
    "elseif" => E{},
    "empty" => E{},
    "enable" => E{},
    "enclosed" => E{},
    "encryption" => E{},
    "end" => E{},
    "ends" => E{},
    "engine" => E{},
    "engines" => E{},
    "enum" => E{},
    "error" => E{},
    "errors" => E{},
    "escape" => E{},
    "escaped" => E{},
    "event" => E{},
    "events" => E{},
    "every" => E{},
    "except" => E{},
    "exchange" => E{},
    "exclude" => E{},
    "execute" => E{},
    "exists" => E{},
    "exit" => E{},
    "expansion" => E{},
    "expire" => E{},
    "explain" => E{},
    "export" => E{},
    "extended" => E{},
    "extent_size" => E{},
    "false" => E{},
    "fast" => E{},
    "faults" => E{},
    "fetch" => E{},
    "fields" => E{},
    "file" => E{},
    "file_block_size" => E{},
    "filter" => E{},
    "first" => E{},
    "first_value" => E{},
    "fixed" => E{},
    "float" => E{},
    "float4" => E{},
    "float8" => E{},
    "flush" => E{},
    "following" => E{},
    "follows" => E{},
    "for" => E{},
    "force" => E{},
    "foreign" => E{},
    "format" => E{},
    "found" => E{},
    "from" => E{},
    "full" => E{},
    "fulltext" => E{},
    "function" => E{},
    "general" => E{},
    "generated" => E{},
    "geomcollection" => E{},
    "geometry" => E{},
    "geometrycollection" => E{},
    "get" => E{},
    "get_format" => E{},
    "get_master_public_key" => E{},
    "global" => E{},
    "grant" => E{},
    "grants" => E{},
    "group" => E{},
    "grouping" => E{},
    "groups" => E{},
    "group_replication" => E{},
    "handler" => E{},
    "hash" => E{},
    "having" => E{},
    "help" => E{},
    "high_priority" => E{},
    "histogram" => E{},
    "history" => E{},
    "host" => E{},
    "hosts" => E{},
    "hour" => E{},
    "hour_microsecond" => E{},
    "hour_minute" => E{},
    "hour_second" => E{},
    "identified" => E{},
    "if" => E{},
    "ignore" => E{},
    "ignore_server_ids" => E{},
    "import" => E{},
    "in" => E{},
    "inactive" => E{},
    "index" => E{},
    "indexes" => E{},
    "infile" => E{},
    "initial_size" => E{},
    "inner" => E{},
    "inout" => E{},
    "insensitive" => E{},
    "insert" => E{},
    "insert_method" => E{},
    "install" => E{},
    "instance" => E{},
    "int" => E{},
    "int1" => E{},
    "int2" => E{},
    "int3" => E{},
    "int4" => E{},
    "int8" => E{},
    "integer" => E{},
    "interval" => E{},
    "into" => E{},
    "invisible" => E{},
    "invoker" => E{},
    "io" => E{},
    "io_after_gtids" => E{},
    "io_before_gtids" => E{},
    "io_thread" => E{},
    "ipc" => E{},
    "is" => E{},
    "isolation" => E{},
    "issuer" => E{},
    "iterate" => E{},
    "join" => E{},
    "json" => E{},
    "json_table" => E{},
    "key" => E{},
    "keys" => E{},
    "key_block_size" => E{},
    "kill" => E{},
    "lag" => E{},
    "language" => E{},
    "last" => E{},
    "last_value" => E{},
    "lateral" => E{},
    "lead" => E{},
    "leading" => E{},
    "leave" => E{},
    "leaves" => E{},
    "left" => E{},
    "less" => E{},
    "level" => E{},
    "like" => E{},
    "limit" => E{},
    "linear" => E{},
    "lines" => E{},
    "linestring" => E{},
    "list" => E{},
    "load" => E{},
    "local" => E{},
    "localtime" => E{},
    "localtimestamp" => E{},
    "lock" => E{},
    "locked" => E{},
    "locks" => E{},
    "logfile" => E{},
    "logs" => E{},
    "long" => E{},
    "longblob" => E{},
    "longtext" => E{},
    "loop" => E{},
    "low_priority" => E{},
    "master" => E{},
    "master_auto_position" => E{},
    "master_bind" => E{},
    "master_connect_retry" => E{},
    "master_delay" => E{},
    "master_heartbeat_period" => E{},
    "master_host" => E{},
    "master_log_file" => E{},
    "master_log_pos" => E{},
    "master_password" => E{},
    "master_port" => E{},
    "master_public_key_path" => E{},
    "master_retry_count" => E{},
    "master_server_id" => E{},
    "master_ssl" => E{},
    "master_ssl_ca" => E{},
    "master_ssl_capath" => E{},
    "master_ssl_cert" => E{},
    "master_ssl_cipher" => E{},
    "master_ssl_crl" => E{},
    "master_ssl_crlpath" => E{},
    "master_ssl_key" => E{},
    "master_ssl_verify_server_cert" => E{},
    "master_tls_version" => E{},
    "master_user" => E{},
    "match" => E{},
    "maxvalue" => E{},
    "max_connections_per_hour" => E{},
    "max_queries_per_hour" => E{},
    "max_rows" => E{},
    "max_size" => E{},
    "max_updates_per_hour" => E{},
    "max_user_connections" => E{},
    "medium" => E{},
    "mediumblob" => E{},
    "mediumint" => E{},
    "mediumtext" => E{},
    "memory" => E{},
    "merge" => E{},
    "message_text" => E{},
    "microsecond" => E{},
    "middleint" => E{},
    "migrate" => E{},
    "minute" => E{},
    "minute_microsecond" => E{},
    "minute_second" => E{},
    "min_rows" => E{},
    "mod" => E{},
    "mode" => E{},
    "modifies" => E{},
    "modify" => E{},
    "month" => E{},
    "multilinestring" => E{},
    "multipoint" => E{},
    "multipolygon" => E{},
    "mutex" => E{},
    "mysql_errno" => E{},
    "name" => E{},
    "names" => E{},
    "national" => E{},
    "natural" => E{},
    "nchar" => E{},
    "ndb" => E{},
    "ndbcluster" => E{},
    "nested" => E{},
    "never" => E{},
    "new" => E{},
    "next" => E{},
    "no" => E{},
    "nodegroup" => E{},
    "none" => E{},
    "not" => E{},
    "nowait" => E{},
    "no_wait" => E{},
    "no_write_to_binlog" => E{},
    "nth_value" => E{},
    "ntile" => E{},
    "null" => E{},
    "nulls" => E{},
    "number" => E{},
    "numeric" => E{},
    "nvarchar" => E{},
    "of" => E{},
    "offset" => E{},
    "old" => E{},
    "on" => E{},
    "one" => E{},
    "only" => E{},
    "open" => E{},
    "optimize" => E{},
    "optimizer_costs" => E{},
    "option" => E{},
    "optional" => E{},
    "optionally" => E{},
    "options" => E{},
    "or" => E{},
    "order" => E{},
    "ordinality" => E{},
    "organization" => E{},
    "others" => E{},
    "out" => E{},
    "outer" => E{},
    "outfile" => E{},
    "over" => E{},
    "owner" => E{},
    "pack_keys" => E{},
    "page" => E{},
    "parser" => E{},
    "partial" => E{},
    "partition" => E{},
    "partitioning" => E{},
    "partitions" => E{},
    "password" => E{},
    "path" => E{},
    "percent_rank" => E{},
    "persist" => E{},
    "persist_only" => E{},
    "phase" => E{},
    "plugin" => E{},
    "plugins" => E{},
    "plugin_dir" => E{},
    "point" => E{},
    "polygon" => E{},
    "port" => E{},
    "precedes" => E{},
    "preceding" => E{},
    "precision" => E{},
    "prepare" => E{},
    "preserve" => E{},
    "prev" => E{},
    "primary" => E{},
    "privileges" => E{},
    "procedure" => E{},
    "process" => E{},
    "processlist" => E{},
    "profile" => E{},
    "profiles" => E{},
    "proxy" => E{},
    "purge" => E{},
    "quarter" => E{},
    "query" => E{},
    "quick" => E{},
    "range" => E{},
    "rank" => E{},
    "read" => E{},
    "reads" => E{},
    "read_only" => E{},
    "read_write" => E{},
    "real" => E{},
    "rebuild" => E{},
    "recover" => E{},
    "recursive" => E{},
    "redofile" => E{},
    "redo_buffer_size" => E{},
    "redundant" => E{},
    "reference" => E{},
    "references" => E{},
    "regexp" => E{},
    "relay" => E{},
    "relaylog" => E{},
    "relay_log_file" => E{},
    "relay_log_pos" => E{},
    "relay_thread" => E{},
    "release" => E{},
    "reload" => E{},
    "remote" => E{},
    "remove" => E{},
    "rename" => E{},
    "reorganize" => E{},
    "repair" => E{},
    "repeat" => E{},
    "repeatable" => E{},
    "replace" => E{},
    "replicate_do_db" => E{},
    "replicate_do_table" => E{},
    "replicate_ignore_db" => E{},
    "replicate_ignore_table" => E{},
    "replicate_rewrite_db" => E{},
    "replicate_wild_do_table" => E{},
    "replicate_wild_ignore_table" => E{},
    "replication" => E{},
    "require" => E{},
    "reset" => E{},
    "resignal" => E{},
    "resource" => E{},
    "respect" => E{},
    "restart" => E{},
    "restore" => E{},
    "restrict" => E{},
    "resume" => E{},
    "retain" => E{},
    "return" => E{},
    "returned_sqlstate" => E{},
    "returns" => E{},
    "reuse" => E{},
    "reverse" => E{},
    "revoke" => E{},
    "right" => E{},
    "rlike" => E{},
    "role" => E{},
    "rollback" => E{},
    "rollup" => E{},
    "rotate" => E{},
    "routine" => E{},
    "row" => E{},
    "rows" => E{},
    "row_count" => E{},
    "row_format" => E{},
    "row_number" => E{},
    "rtree" => E{},
    "savepoint" => E{},
    "schedule" => E{},
    "schema" => E{},
    "schemas" => E{},
    "schema_name" => E{},
    "second" => E{},
    "secondary_engine" => E{},
    "secondary_load" => E{},
    "secondary_unload" => E{},
    "second_microsecond" => E{},
    "security" => E{},
    "select" => E{},
    "sensitive" => E{},
    "separator" => E{},
    "serial" => E{},
    "serializable" => E{},
    "server" => E{},
    "session" => E{},
    "set" => E{},
    "share" => E{},
    "show" => E{},
    "shutdown" => E{},
    "signal" => E{},
    "signed" => E{},
    "simple" => E{},
    "skip" => E{},
    "slave" => E{},
    "slow" => E{},
    "smallint" => E{},
    "snapshot" => E{},
    "socket" => E{},
    "some" => E{},
    "soname" => E{},
    "sounds" => E{},
    "source" => E{},
    "spatial" => E{},
    "specific" => E{},
    "sql" => E{},
    "sqlexception" => E{},
    "sqlstate" => E{},
    "sqlwarning" => E{},
    "sql_after_gtids" => E{},
    "sql_after_mts_gaps" => E{},
    "sql_before_gtids" => E{},
    "sql_big_result" => E{},
    "sql_buffer_result" => E{},
    "sql_cache" => E{},
    "sql_calc_found_rows" => E{},
    "sql_no_cache" => E{},
    "sql_small_result" => E{},
    "sql_thread" => E{},
    "sql_tsi_day" => E{},
    "sql_tsi_hour" => E{},
    "sql_tsi_minute" => E{},
    "sql_tsi_month" => E{},
    "sql_tsi_quarter" => E{},
    "sql_tsi_second" => E{},
    "sql_tsi_week" => E{},
    "sql_tsi_year" => E{},
    "srid" => E{},
    "ssl" => E{},
    "stacked" => E{},
    "start" => E{},
    "starting" => E{},
    "starts" => E{},
    "stats_auto_recalc" => E{},
    "stats_persistent" => E{},
    "stats_sample_pages" => E{},
    "status" => E{},
    "stop" => E{},
    "storage" => E{},
    "stored" => E{},
    "straight_join" => E{},
    "string" => E{},
    "subclass_origin" => E{},
    "subject" => E{},
    "subpartition" => E{},
    "subpartitions" => E{},
    "super" => E{},
    "suspend" => E{},
    "swaps" => E{},
    "switches" => E{},
    "system" => E{},
    "table" => E{},
    "tables" => E{},
    "tablespace" => E{},
    "table_checksum" => E{},
    "table_name" => E{},
    "temporary" => E{},
    "temptable" => E{},
    "terminated" => E{},
    "text" => E{},
    "than" => E{},
    "then" => E{},
    "thread_priority" => E{},
    "ties" => E{},
    "time" => E{},
    "timestamp" => E{},
    "timestampadd" => E{},
    "timestampdiff" => E{},
    "tinyblob" => E{},
    "tinyint" => E{},
    "tinytext" => E{},
    "to" => E{},
    "trailing" => E{},
    "transaction" => E{},
    "trigger" => E{},
    "triggers" => E{},
    "true" => E{},
    "truncate" => E{},
    "type" => E{},
    "types" => E{},
    "unbounded" => E{},
    "uncommitted" => E{},
    "undefined" => E{},
    "undo" => E{},
    "undofile" => E{},
    "undo_buffer_size" => E{},
    "unicode" => E{},
    "uninstall" => E{},
    "union" => E{},
    "unique" => E{},
    "unknown" => E{},
    "unlock" => E{},
    "unsigned" => E{},
    "until" => E{},
    "update" => E{},
    "upgrade" => E{},
    "usage" => E{},
    "use" => E{},
    "user" => E{},
    "user_resources" => E{},
    "use_frm" => E{},
    "using" => E{},
    "utc_date" => E{},
    "utc_time" => E{},
    "utc_timestamp" => E{},
    "validation" => E{},
    "value" => E{},
    "values" => E{},
    "varbinary" => E{},
    "varchar" => E{},
    "varcharacter" => E{},
    "variables" => E{},
    "varying" => E{},
    "vcpu" => E{},
    "view" => E{},
    "virtual" => E{},
    "visible" => E{},
    "wait" => E{},
    "warnings" => E{},
    "week" => E{},
    "weight_string" => E{},
    "when" => E{},
    "where" => E{},
    "while" => E{},
    "window" => E{},
    "with" => E{},
    "without" => E{},
    "work" => E{},
    "wrapper" => E{},
    "write" => E{},
    "x509" => E{},
    "xa" => E{},
    "xid" => E{},
    "xml" => E{},
    "xor" => E{},
    "year" => E{},
    "year_month" => E{},
    "zerofill" => E{},
        };
    let functions = hashmap!{
    "abs" => E{},
    "acos" => E{},
    "adddate" => E{},
    "addtime" => E{},
    "aes_decrypt" => E{},
    "aes_encrypt" => E{},
    "any_value" => E{},
    "ascii" => E{},
    "asin" => E{},
    "asymmetric_decrypt" => E{},
    "asymmetric_derive" => E{},
    "asymmetric_encrypt" => E{},
    "asymmetric_sign" => E{},
    "asymmetric_verify" => E{},
    "atan" => E{},
    "atan2" => E{},
    "avg" => E{},
    "benchmark" => E{},
    "bin" => E{},
    "bin_to_uuid" => E{},
    "bit_and" => E{},
    "bit_count" => E{},
    "bit_length" => E{},
    "bit_or" => E{},
    "bit_xor" => E{},
    "can_access_column" => E{},
    "can_access_database" => E{},
    "can_access_table" => E{},
    "can_access_view" => E{},
    "cast" => E{},
    "ceil" => E{},
    "ceiling" => E{},
    "char" => E{},
    "char_length" => E{},
    "character_length" => E{},
    "charset" => E{},
    "coalesce" => E{},
    "coercibility" => E{},
    "collation" => E{},
    "compress" => E{},
    "concat" => E{},
    "concat_ws" => E{},
    "connection_id" => E{},
    "conv" => E{},
    "convert" => E{},
    "convert_tz" => E{},
    "cos" => E{},
    "cot" => E{},
    "count" => E{},
    "crc32" => E{},
    "create_asymmetric_priv_key" => E{},
    "create_asymmetric_pub_key" => E{},
    "create_dh_parameters" => E{},
    "create_digest" => E{},
    "cume_dist" => E{},
    "curdate" => E{},
    "current_date" => E{},
    "current_role" => E{},
    "current_time" => E{},
    "current_timestamp" => E{},
    "current_user" => E{},
    "curtime" => E{},
    "database" => E{},
    "date" => E{},
    "date_add" => E{},
    "date_format" => E{},
    "date_sub" => E{},
    "datediff" => E{},
    "day" => E{},
    "dayname" => E{},
    "dayofmonth" => E{},
    "dayofweek" => E{},
    "dayofyear" => E{},
    "decode" => E{},
    "default" => E{},
    "degrees" => E{},
    "dense_rank" => E{},
    "des_decrypt" => E{},
    "des_encrypt" => E{},
    "elt" => E{},
    "encode" => E{},
    "encrypt" => E{},
    "exp" => E{},
    "export_set" => E{},
    "extract" => E{},
    "extractvalue" => E{},
    "field" => E{},
    "find_in_set" => E{},
    "first_value" => E{},
    "floor" => E{},
    "format" => E{},
    "found_rows" => E{},
    "from_base64" => E{},
    "from_days" => E{},
    "from_unixtime" => E{},
    "geomcollection" => E{},
    "geometrycollection" => E{},
    "get_dd_column_privileges" => E{},
    "get_dd_create_options" => E{},
    "get_dd_index_sub_part_length" => E{},
    "get_format" => E{},
    "get_lock" => E{},
    "greatest" => E{},
    "group_concat" => E{},
    "grouping" => E{},
    "gtid_subset" => E{},
    "gtid_subtract" => E{},
    "hex" => E{},
    "hour" => E{},
    "icu_version" => E{},
    "if" => E{},
    "ifnull" => E{},
    "in" => E{},
    "inet_aton" => E{},
    "inet_ntoa" => E{},
    "inet6_aton" => E{},
    "inet6_ntoa" => E{},
    "instr" => E{},
    "internal_auto_increment" => E{},
    "internal_avg_row_length" => E{},
    "internal_check_time" => E{},
    "internal_checksum" => E{},
    "internal_data_free" => E{},
    "internal_data_length" => E{},
    "internal_dd_char_length" => E{},
    "internal_get_comment_or_error" => E{},
    "internal_get_view_warning_or_error" => E{},
    "internal_index_column_cardinality" => E{},
    "internal_index_length" => E{},
    "internal_keys_disabled" => E{},
    "internal_max_data_length" => E{},
    "internal_table_rows" => E{},
    "internal_update_time" => E{},
    "interval" => E{},
    "is_free_lock" => E{},
    "is_ipv4" => E{},
    "is_ipv4_compat" => E{},
    "is_ipv4_mapped" => E{},
    "is_ipv6" => E{},
    "is_used_lock" => E{},
    "is_uuid" => E{},
    "isnull" => E{},
    "json_array" => E{},
    "json_array_append" => E{},
    "json_array_insert" => E{},
    "json_arrayagg" => E{},
    "json_contains" => E{},
    "json_contains_path" => E{},
    "json_depth" => E{},
    "json_extract" => E{},
    "json_insert" => E{},
    "json_keys" => E{},
    "json_length" => E{},
    "json_merge" => E{},
    "json_merge_patch" => E{},
    "json_merge_preserve" => E{},
    "json_object" => E{},
    "json_objectagg" => E{},
    "json_pretty" => E{},
    "json_quote" => E{},
    "json_remove" => E{},
    "json_replace" => E{},
    "json_search" => E{},
    "json_set" => E{},
    "json_storage_free" => E{},
    "json_storage_size" => E{},
    "json_table" => E{},
    "json_type" => E{},
    "json_unquote" => E{},
    "json_valid" => E{},
    "lag" => E{},
    "last_insert_id" => E{},
    "last_value" => E{},
    "lcase" => E{},
    "lead" => E{},
    "least" => E{},
    "left" => E{},
    "length" => E{},
    "linestring" => E{},
    "ln" => E{},
    "load_file" => E{},
    "localtime" => E{},
    "localtimestamp" => E{},
    "locate" => E{},
    "log" => E{},
    "log10" => E{},
    "log2" => E{},
    "lower" => E{},
    "lpad" => E{},
    "ltrim" => E{},
    "make_set" => E{},
    "makedate" => E{},
    "maketime" => E{},
    "master_pos_wait" => E{},
    "max" => E{},
    "mbrcontains" => E{},
    "mbrcoveredby" => E{},
    "mbrcovers" => E{},
    "mbrdisjoint" => E{},
    "mbrequals" => E{},
    "mbrintersects" => E{},
    "mbroverlaps" => E{},
    "mbrtouches" => E{},
    "mbrwithin" => E{},
    "md5" => E{},
    "microsecond" => E{},
    "mid" => E{},
    "min" => E{},
    "minute" => E{},
    "mod" => E{},
    "month" => E{},
    "monthname" => E{},
    "multilinestring" => E{},
    "multipoint" => E{},
    "multipolygon" => E{},
    "name_const" => E{},
    "now" => E{},
    "nth_value" => E{},
    "ntile" => E{},
    "nullif" => E{},
    "oct" => E{},
    "octet_length" => E{},
    "ord" => E{},
    "password" => E{},
    "percent_rank" => E{},
    "period_add" => E{},
    "period_diff" => E{},
    "pi" => E{},
    "point" => E{},
    "polygon" => E{},
    "position" => E{},
    "pow" => E{},
    "power" => E{},
    "quarter" => E{},
    "quote" => E{},
    "radians" => E{},
    "rand" => E{},
    "random_bytes" => E{},
    "rank" => E{},
    "regexp_instr" => E{},
    "regexp_like" => E{},
    "regexp_replace" => E{},
    "regexp_substr" => E{},
    "release_all_locks" => E{},
    "release_lock" => E{},
    "repeat" => E{},
    "replace" => E{},
    "reverse" => E{},
    "right" => E{},
    "roles_graphml" => E{},
    "round" => E{},
    "row_count" => E{},
    "row_number" => E{},
    "rpad" => E{},
    "rtrim" => E{},
    "schema" => E{},
    "sec_to_time" => E{},
    "second" => E{},
    "session_user" => E{},
    "sha1" => E{},
    "sha" => E{},
    "sha2" => E{},
    "sign" => E{},
    "sin" => E{},
    "sleep" => E{},
    "soundex" => E{},
    "space" => E{},
    "sqrt" => E{},
    "st_area" => E{},
    "st_asbinary" => E{},
    "st_aswkb" => E{},
    "st_asgeojson" => E{},
    "st_astext" => E{},
    "st_aswkt" => E{},
    "st_buffer" => E{},
    "st_buffer_strategy" => E{},
    "st_centroid" => E{},
    "st_contains" => E{},
    "st_convexhull" => E{},
    "st_crosses" => E{},
    "st_difference" => E{},
    "st_dimension" => E{},
    "st_disjoint" => E{},
    "st_distance" => E{},
    "st_distance_sphere" => E{},
    "st_endpoint" => E{},
    "st_envelope" => E{},
    "st_equals" => E{},
    "st_exteriorring" => E{},
    "st_geohash" => E{},
    "st_geomcollfromtext" => E{},
    "st_geometrycollectionfromtext" => E{},
    "st_geomcollfromtxt" => E{},
    "st_geomcollfromwkb" => E{},
    "st_geometrycollectionfromwkb" => E{},
    "st_geometryn" => E{},
    "st_geometrytype" => E{},
    "st_geomfromgeojson" => E{},
    "st_geomfromtext" => E{},
    "st_geometryfromtext" => E{},
    "st_geomfromwkb" => E{},
    "st_geometryfromwkb" => E{},
    "st_interiorringn" => E{},
    "st_intersection" => E{},
    "st_intersects" => E{},
    "st_isclosed" => E{},
    "st_isempty" => E{},
    "st_issimple" => E{},
    "st_isvalid" => E{},
    "st_latfromgeohash" => E{},
    "st_latitude" => E{},
    "st_length" => E{},
    "st_linefromtext" => E{},
    "st_linestringfromtext" => E{},
    "st_linefromwkb" => E{},
    "st_linestringfromwkb" => E{},
    "st_longfromgeohash" => E{},
    "st_longitude" => E{},
    "st_makeenvelope" => E{},
    "st_mlinefromtext" => E{},
    "st_multilinestringfromtext" => E{},
    "st_mlinefromwkb" => E{},
    "st_multilinestringfromwkb" => E{},
    "st_mpointfromtext" => E{},
    "st_multipointfromtext" => E{},
    "st_mpointfromwkb" => E{},
    "st_multipointfromwkb" => E{},
    "st_mpolyfromtext" => E{},
    "st_multipolygonfromtext" => E{},
    "st_mpolyfromwkb" => E{},
    "st_multipolygonfromwkb" => E{},
    "st_numgeometries" => E{},
    "st_numinteriorring" => E{},
    "st_numinteriorrings" => E{},
    "st_numpoints" => E{},
    "st_overlaps" => E{},
    "st_pointfromgeohash" => E{},
    "st_pointfromtext" => E{},
    "st_pointfromwkb" => E{},
    "st_pointn" => E{},
    "st_polyfromtext" => E{},
    "st_polygonfromtext" => E{},
    "st_polyfromwkb" => E{},
    "st_polygonfromwkb" => E{},
    "st_simplify" => E{},
    "st_srid" => E{},
    "st_startpoint" => E{},
    "st_swapxy" => E{},
    "st_symdifference" => E{},
    "st_touches" => E{},
    "st_transform" => E{},
    "st_union" => E{},
    "st_validate" => E{},
    "st_within" => E{},
    "st_x" => E{},
    "st_y" => E{},
    "statement_digest" => E{},
    "statement_digest_text" => E{},
    "std" => E{},
    "stddev" => E{},
    "stddev_pop" => E{},
    "stddev_samp" => E{},
    "str_to_date" => E{},
    "strcmp" => E{},
    "subdate" => E{},
    "substr" => E{},
    "substring" => E{},
    "substring_index" => E{},
    "subtime" => E{},
    "sum" => E{},
    "sysdate" => E{},
    "system_user" => E{},
    "tan" => E{},
    "time" => E{},
    "time_format" => E{},
    "time_to_sec" => E{},
    "timediff" => E{},
    "timestamp" => E{},
    "timestampadd" => E{},
    "timestampdiff" => E{},
    "to_base64" => E{},
    "to_days" => E{},
    "to_seconds" => E{},
    "trim" => E{},
    "truncate" => E{},
    "ucase" => E{},
    "uncompress" => E{},
    "uncompressed_length" => E{},
    "unhex" => E{},
    "unix_timestamp" => E{},
    "updatexml" => E{},
    "upper" => E{},
    "user" => E{},
    "utc_date" => E{},
    "utc_time" => E{},
    "utc_timestamp" => E{},
    "uuid" => E{},
    "uuid_short" => E{},
    "uuid_to_bin" => E{},
    "validate_password_strength" => E{},
    "values" => E{},
    "var_pop" => E{},
    "var_samp" => E{},
    "variance" => E{},
    "version" => E{},
    "wait_for_executed_gtid_set" => E{},
    "wait_until_sql_thread_after_gtids" => E{},
    "week" => E{},
    "weekday" => E{},
    "weekofyear" => E{},
    "weight_string" => E{},
    "year" => E{},
    "yearweek" => E{},
        };

    let bytes = MYSQL.as_bytes();
    let mut s = String::with_capacity(bytes.len() * 10);
    s.push_str("<pre><code>");
    let len = MYSQL.len();
    let mut cur_token = Token::Unset;
    let mut prev_token = Token::Unset;
    let mut token_start = 0usize;
    let mut token_end = 0usize;
    let mut escaped = false;
    let mut quote = '\'';
    let mut p = 0;
    let mut prev_str = String::new();
    let mut word = String::new();
    let mut skip = 0u8;

    let start = PreciseTime::now();
    for (i, b) in bytes.iter().enumerate() {
        if skip != 0 {
            skip -= 1;
            continue;
        }

        macro_rules! push_newline {
            () => {
                s.push_str(&format!("\n{}", " ".repeat(2 * p)));
            };
        }

        // push a token to the formatted output
        macro_rules! push_token {
            () => {
                match cur_token {
                    Token::Unset => {
                        break;
                    }
                    Token::Word => {
                        match prev_token {
                            Token::Word | Token::Number => s.push(' '),
                            _ => {}
                        }
                        match word.as_str() {
                            "from" | "where" | "and" | "or" | "order" | "group" | "having"
                            | "limit" | "straight_join" | "cross" | "natural" | "union" => {
                                push_newline!()
                            }
                            // "select" => match prev_token {
                            //     Token::Symbol => {
                            //         if prev_char == '(' {
                            //             push_newline!();
                            //         }
                            //     }
                            //     _ => {}
                            // },
                            "left" | "right" => match prev_token {
                                Token::Word => match prev_str.as_str() {
                                    "natural" => {}
                                    _ => push_newline!(),
                                },
                                _ => push_newline!(),
                            },
                            "join" => match prev_token {
                                Token::Word => match prev_str.as_str() {
                                    "inner" | "cross" | "outer" | "left" | "right"
                                    | "natural" => {}
                                    _ => push_newline!(),
                                },
                                _ => push_newline!(),
                            },
                            _ => {}
                        }
                        s.push_str("<b style=\"color:#1976d2\">");
                        s.push_str(&word);
                        s.push_str("</b>");
                        prev_str = word.clone();
                    }
                    Token::Function => {
                        match prev_token {
                            Token::Word | Token::Number => s.push(' '),
                            _ => {}
                        }
                        s.push_str("<span style=\"color:#d81b60\">");
                        s.push_str(&word);
                        s.push_str("</span>");
                    }
                    Token::String => {
                        s.push_str("<span style=\"color:#2e7d32\">'");
                        if quote == '\'' {
                            s.push_str(&encode_minimal(&MYSQL[token_start..=token_end]));
                        } else {
                            let q = &quote.to_string();
                            s.push_str(&encode_minimal(
                                &MYSQL[token_start..=token_end]
                                    .replace(&format!("{}{}", "\\", q).to_owned(), q)
                                    .replace("'", "\\'"),
                            ));
                        }
                        s.push_str("'</span>");
                    }
                    Token::Name => {
                        match prev_token {
                            Token::Name => s.push(' '),
                            _ => {}
                        }
                        s.push_str("<span style=\"color:#5d4037\">`");
                        s.push_str(&encode_minimal(&MYSQL[token_start..=token_end]));
                        s.push_str("`</span>");
                    }
                    Token::Number => {
                        match prev_token {
                            Token::Word => s.push(' '),
                            _ => {}
                        }
                        s.push_str("<span style=\"color:#ce4800\">");
                        s.push_str(&MYSQL[token_start..=token_end]);
                        s.push_str("</span>");
                    }
                    Token::Hex | Token::HexString => {
                        match prev_token {
                            Token::Word => s.push(' '),
                            _ => {}
                        }
                        s.push_str("<span style=\"color:#673ab7;background-color:#f0f0f0\">");
                        s.push_str(&MYSQL[token_start..=token_end]);
                        s.push_str("</span>");
                    }
                    Token::Symbol => {
                        s.push_str("<b>");
                        s.push_str(&encode_minimal(&format!("{}", bytes[i] as char)));
                        s.push_str("</b>");

                        // prev_char = bytes[i] as char;
                    }
                }
                prev_token = cur_token;
                cur_token = Token::Unset;
            };
        }

        // make sure current byte is valid for cur_token
        match cur_token {
            Token::Unset => {}
            Token::Word | Token::Function => match *b as char {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {}
                _ => {
                    token_end = i - 1;
                    word = MYSQL[token_start..=token_end].to_ascii_lowercase();
                    if *b as char == '(' {
                        if functions.contains_key(word.as_str()) {
                            cur_token = Token::Function;
                        } else if !reserved.contains_key(word.as_str()) {
                            cur_token = Token::Name;
                        }
                    } else if !reserved.contains_key(word.as_str()) {
                        cur_token = Token::Name;
                    }
                    // match word.as_str() {
                    //      => {}
                    //      => cur_token = Token::Function,
                    //     _ => cur_token = Token::Name,
                    // }
                    push_token!();
                }
            },
            Token::Number => match *b as char {
                '0'...'9' | '.' => {}
                _ => {
                    token_end = i - 1;
                    push_token!();
                }
            },
            Token::Name | Token::String => {
                if !escaped {
                    match *b as char {
                        b if b == quote => {
                            token_end = i - 1;
                            push_token!();
                            continue;
                        }
                        '\\' => escaped = true,
                        _ => {}
                    }
                } else {
                    escaped = false;
                }
            }
            Token::HexString => match *b as char {
                '\'' => {
                    token_end = i - 1;
                    push_token!();
                    continue;
                }
                _ => {}
            },
            Token::Symbol => {}
            Token::Hex => match *b as char {
                'a'...'f' | 'A'...'F' | '0'...'9' => {}
                _ => {
                    token_end = i - 1;
                    push_token!();
                }
            },
        }

        // Set token if we're currently unset
        match cur_token {
            Token::Unset => match *b as char {
                'a'...'z' | 'A'...'Z' => {
                    match *b as char {
                        'x' | 'X' => {
                            if i + 1 < len {
                                let next_b = bytes[i + 1] as char;
                                if next_b == '\'' {
                                    cur_token = Token::HexString;
                                    token_start = i;
                                    skip = 1;
                                }
                            }

                            continue;
                        }
                        _ => {}
                    }

                    cur_token = Token::Word;
                    token_start = i;
                }
                '`' => {
                    cur_token = Token::Name;
                    token_start = i + 1;
                    quote = '`';
                }
                '\'' | '"' => {
                    cur_token = Token::String;
                    token_start = i + 1;
                    quote = *b as char
                }
                '0'...'9' => {
                    match *b as char {
                        '0' => {
                            if i + 1 < len {
                                let next_b = bytes[i + 1] as char;
                                if next_b == 'x' || next_b == 'X' {
                                    cur_token = Token::Hex;
                                    token_start = i;
                                    skip = 1;
                                }
                            }

                            continue;
                        }
                        _ => {}
                    }

                    cur_token = Token::Number;
                    token_start = i;
                }
                '.' | '=' | ';' | '(' | ')' | '?' | '^' | '&' | '|' | '/' | '*' | '+' | '-'
                | ':' | '~' | '<' | '>' | '!' | '%' | ',' => {
                    match *b as char {
                        '(' => p += 1,
                        ')' => p -= 1,
                        _ => {}
                    }
                    cur_token = Token::Symbol;
                    push_token!();
                }
                c if c.is_whitespace() => {}
                _ => {
                    println!("Unexpected byte `{}` at pos {}", *b as char, i);
                    break;
                }
            },
            _ => {}
        }

        if len == i + 1 {
            token_end = i;
            push_token!();
        }
    }
    let end = PreciseTime::now();

    s.push_str("</code></pre>");

    println!("{}", s);
    println!("Took {}", start.to(end));

    fs::write(
        "/home/brian/rust/src/github.com/BrianLeishman/mysql-format/debug.html",
        s,
    ).expect("Unable to write file");
}
