extern crate libc;
extern crate openssl_sys;

#[allow(non_camel_case_types, non_snake_case)]
pub mod bindings {
    // Bindings are based on the ones generated by rust-bindgen. These can be generated like this:
    // bindgen -I ~/path/mongo-rust-driver/mongoc-sys/mongo-c-driver-{version}/src/libbson/src/bson ~/path/mongo-rust-driver/mongoc-sys/mongo-c-driver-{version}/src/mongoc/mongoc.h

    pub type int8_t = ::libc::c_char;
    pub type int16_t = ::libc::c_short;
    pub type int32_t = ::libc::c_int;
    pub type int64_t = ::libc::c_longlong;
    pub type uint8_t = ::libc::c_uchar;
    pub type uint16_t = ::libc::c_ushort;
    pub type uint32_t = ::libc::c_uint;
    pub type size_t = ::libc::c_ulong;

    // Libbson
    #[repr(C)]
    #[derive(Copy)]
    pub struct bson_t {
        pub flags: uint32_t,
        pub len: uint32_t,
        pub padding: [uint8_t; 120usize],
    }
    impl Clone for bson_t {
        fn clone(&self) -> Self {
            *self
        }
    }
    #[repr(C)]
    pub struct bson_reader_t {
        pub _type: uint32_t,
    }
    #[repr(C)]
    pub struct bson_error_t {
        pub domain: uint32_t,
        pub code: uint32_t,
        pub message: [::libc::c_char; 504usize],
    }
    extern "C" {
        pub fn bson_as_json(bson: *const bson_t, length: *mut size_t) -> *mut ::libc::c_char;
        pub fn bson_destroy(bson: *mut bson_t) -> ();
        pub fn bson_free(mem: *mut ::libc::c_void) -> ();
        pub fn bson_get_data(bson: *const bson_t) -> *const uint8_t;
        pub fn bson_new() -> *mut bson_t;
        pub fn bson_new_from_data(data: *const uint8_t, length: size_t) -> *mut bson_t;
        pub fn bson_reader_new_from_data(
            data: *const uint8_t,
            length: size_t,
        ) -> *mut bson_reader_t;
    }

    // Init and logging
    pub type mongoc_log_level_t = ::libc::c_uint;
    pub type mongoc_log_func_t = Option<
        unsafe extern "C" fn(
            log_level: mongoc_log_level_t,
            log_domain: *const ::libc::c_char,
            message: *const ::libc::c_char,
            user_data: *mut ::libc::c_void,
        ) -> (),
    >;
    extern "C" {
        pub fn mongoc_init() -> ();
        pub fn mongoc_log_set_handler(
            log_func: mongoc_log_func_t,
            user_data: *mut ::libc::c_void,
        ) -> ();
    }
    pub const MONGOC_LOG_LEVEL_ERROR: ::libc::c_uint = 0;
    pub const MONGOC_LOG_LEVEL_CRITICAL: ::libc::c_uint = 1;
    pub const MONGOC_LOG_LEVEL_WARNING: ::libc::c_uint = 2;
    pub const MONGOC_LOG_LEVEL_MESSAGE: ::libc::c_uint = 3;
    pub const MONGOC_LOG_LEVEL_INFO: ::libc::c_uint = 4;
    pub const MONGOC_LOG_LEVEL_DEBUG: ::libc::c_uint = 5;
    pub const MONGOC_LOG_LEVEL_TRACE: ::libc::c_uint = 6;

    // Read prefs
    pub enum mongoc_read_prefs_t {}
    pub type mongoc_read_mode_t = ::libc::c_uint;
    pub const MONGOC_READ_PRIMARY: ::libc::c_uint = 1;
    pub const MONGOC_READ_SECONDARY: ::libc::c_uint = 2;
    pub const MONGOC_READ_PRIMARY_PREFERRED: ::libc::c_uint = 5;
    pub const MONGOC_READ_SECONDARY_PREFERRED: ::libc::c_uint = 6;
    pub const MONGOC_READ_NEAREST: ::libc::c_uint = 10;
    extern "C" {
        pub fn mongoc_read_prefs_new(read_mode: mongoc_read_mode_t) -> *mut mongoc_read_prefs_t;
        pub fn mongoc_read_prefs_destroy(read_prefs: *mut mongoc_read_prefs_t) -> ();
    }

    // Uri
    pub enum mongoc_uri_t {}
    extern "C" {
        pub fn mongoc_uri_new(uri_string: *const ::libc::c_char) -> *mut mongoc_uri_t;
        pub fn mongoc_uri_get_string(uri: *const mongoc_uri_t) -> *const ::libc::c_char;
        pub fn mongoc_uri_get_database(uri: *const mongoc_uri_t) -> *const ::libc::c_char;
        pub fn mongoc_uri_destroy(uri: *mut mongoc_uri_t) -> ();
    }

    // Write concern
    pub enum mongoc_write_concern_t {}
    extern "C" {
        pub fn mongoc_write_concern_new() -> *mut mongoc_write_concern_t;
        pub fn mongoc_write_concern_destroy(write_concern: *mut mongoc_write_concern_t) -> ();
    }

    // Database
    pub enum mongoc_database_t {}
    extern "C" {
        pub fn mongoc_database_command(
            database: *mut mongoc_database_t,
            flags: mongoc_query_flags_t,
            skip: uint32_t,
            limit: uint32_t,
            batch_size: uint32_t,
            command: *const bson_t,
            fields: *const bson_t,
            read_prefs: *const mongoc_read_prefs_t,
        ) -> *mut mongoc_cursor_t;
        pub fn mongoc_database_command_simple(
            database: *mut mongoc_database_t,
            command: *const bson_t,
            read_prefs: *const mongoc_read_prefs_t,
            reply: *mut bson_t,
            error: *mut bson_error_t,
        ) -> u8;
        pub fn mongoc_database_create_collection(
            database: *mut mongoc_database_t,
            name: *const ::libc::c_char,
            options: *const bson_t,
            error: *mut bson_error_t,
        ) -> *mut mongoc_collection_t;
        pub fn mongoc_database_get_collection(
            database: *mut mongoc_database_t,
            name: *const ::libc::c_char,
        ) -> *mut mongoc_collection_t;
        pub fn mongoc_database_get_name(database: *mut mongoc_database_t) -> *const ::libc::c_char;
        pub fn mongoc_database_destroy(database: *mut mongoc_database_t) -> ();
        pub fn mongoc_database_has_collection(
            database: *mut mongoc_database_t,
            name: *const ::libc::c_char,
            error: *mut bson_error_t,
        ) -> i32;
    }

    // Client
    pub enum mongoc_client_pool_t {}
    pub enum mongoc_client_t {}
    #[repr(C)]
    #[derive(Copy)]
    pub struct mongoc_ssl_opt_t {
        pub pem_file: *const ::libc::c_char,
        pub pem_pwd: *const ::libc::c_char,
        pub ca_file: *const ::libc::c_char,
        pub ca_dir: *const ::libc::c_char,
        pub crl_file: *const ::libc::c_char,
        pub weak_cert_validation: u8,
        pub padding: [*mut ::libc::c_void; 8usize],
    }
    impl Clone for mongoc_ssl_opt_t {
        fn clone(&self) -> Self {
            *self
        }
    }
    extern "C" {
        pub fn mongoc_client_pool_new(uri: *const mongoc_uri_t) -> *mut mongoc_client_pool_t;
        pub fn mongoc_client_pool_set_ssl_opts(
            pool: *mut mongoc_client_pool_t,
            opts: *const mongoc_ssl_opt_t,
        ) -> ();
        pub fn mongoc_client_pool_pop(pool: *mut mongoc_client_pool_t) -> *mut mongoc_client_t;
        pub fn mongoc_client_pool_push(
            pool: *mut mongoc_client_pool_t,
            client: *mut mongoc_client_t,
        ) -> ();
        pub fn mongoc_client_pool_destroy(pool: *mut mongoc_client_pool_t) -> ();
        pub fn mongoc_client_get_collection(
            client: *mut mongoc_client_t,
            db: *const ::libc::c_char,
            collection: *const ::libc::c_char,
        ) -> *mut mongoc_collection_t;
        pub fn mongoc_client_get_database(
            client: *mut mongoc_client_t,
            name: *const ::libc::c_char,
        ) -> *mut mongoc_database_t;
        pub fn mongoc_client_get_server_status(
            client: *mut mongoc_client_t,
            read_prefs: *mut mongoc_read_prefs_t,
            reply: *mut bson_t,
            error: *mut bson_error_t,
        ) -> u8;
        pub fn mongoc_client_read_command_with_opts(
            client: *mut mongoc_client_t,
            db: *const ::libc::c_char,
            command: *const bson_t,
            read_prefs: *const mongoc_read_prefs_t,
            opts: *const bson_t,
            reply: *mut bson_t,
            error: *mut bson_error_t,
        ) -> u8;
    }

    // Collection
    pub enum mongoc_collection_t {}
    pub type mongoc_insert_flags_t = ::libc::c_uint;
    pub type mongoc_remove_flags_t = ::libc::c_uint;
    pub type mongoc_update_flags_t = ::libc::c_uint;
    extern "C" {
        pub fn mongoc_collection_aggregate(
            collection: *mut mongoc_collection_t,
            flags: mongoc_query_flags_t,
            pipeline: *const bson_t,
            options: *const bson_t,
            read_prefs: *const mongoc_read_prefs_t,
        ) -> *mut mongoc_cursor_t;
        pub fn mongoc_collection_command(
            collection: *mut mongoc_collection_t,
            flags: mongoc_query_flags_t,
            skip: uint32_t,
            limit: uint32_t,
            batch_size: uint32_t,
            command: *const bson_t,
            fields: *const bson_t,
            read_prefs: *const mongoc_read_prefs_t,
        ) -> *mut mongoc_cursor_t;
        pub fn mongoc_collection_command_simple(
            collection: *mut mongoc_collection_t,
            command: *const bson_t,
            read_prefs: *const mongoc_read_prefs_t,
            reply: *mut bson_t,
            error: *mut bson_error_t,
        ) -> u8;
        pub fn mongoc_collection_count_with_opts(
            collection: *mut mongoc_collection_t,
            flags: mongoc_query_flags_t,
            query: *const bson_t,
            skip: int64_t,
            limit: int64_t,
            opts: *const bson_t,
            read_prefs: *const mongoc_read_prefs_t,
            error: *mut bson_error_t,
        ) -> int64_t;
        pub fn mongoc_collection_create_bulk_operation(
            collection: *mut mongoc_collection_t,
            ordered: u8,
            write_concern: *const mongoc_write_concern_t,
        ) -> *mut mongoc_bulk_operation_t;
        pub fn mongoc_collection_create_bulk_operation_with_opts(
            collection: *mut mongoc_collection_t,
            opts: *const bson_t,
        ) -> *mut mongoc_bulk_operation_t;
        pub fn mongoc_collection_drop(
            collection: *mut mongoc_collection_t,
            error: *mut bson_error_t,
        ) -> u8;
        pub fn mongoc_collection_find(
            collection: *mut mongoc_collection_t,
            flags: mongoc_query_flags_t,
            skip: uint32_t,
            limit: uint32_t,
            batch_size: uint32_t,
            query: *const bson_t,
            fields: *const bson_t,
            read_prefs: *const mongoc_read_prefs_t,
        ) -> *mut mongoc_cursor_t;
        pub fn mongoc_collection_find_and_modify(
            collection: *mut mongoc_collection_t,
            query: *const bson_t,
            sort: *const bson_t,
            update: *const bson_t,
            fields: *const bson_t,
            _remove: u8,
            upsert: u8,
            _new: u8,
            reply: *mut bson_t,
            error: *mut bson_error_t,
        ) -> u8;
        pub fn mongoc_collection_get_name(
            collection: *mut mongoc_collection_t,
        ) -> *const ::libc::c_char;
        pub fn mongoc_collection_insert(
            collection: *mut mongoc_collection_t,
            flags: mongoc_insert_flags_t,
            document: *const bson_t,
            write_concern: *const mongoc_write_concern_t,
            error: *mut bson_error_t,
        ) -> u8;
        pub fn mongoc_collection_remove(
            collection: *mut mongoc_collection_t,
            flags: mongoc_remove_flags_t,
            selector: *const bson_t,
            write_concern: *const mongoc_write_concern_t,
            error: *mut bson_error_t,
        ) -> u8;
        pub fn mongoc_collection_save(
            collection: *mut mongoc_collection_t,
            document: *const bson_t,
            write_concern: *const mongoc_write_concern_t,
            error: *mut bson_error_t,
        ) -> u8;
        pub fn mongoc_collection_update(
            collection: *mut mongoc_collection_t,
            flags: mongoc_update_flags_t,
            selector: *const bson_t,
            update: *const bson_t,
            write_concern: *const mongoc_write_concern_t,
            error: *mut bson_error_t,
        ) -> u8;
        pub fn mongoc_collection_destroy(collection: *mut mongoc_collection_t) -> ();
    }

    // Cursor
    pub enum mongoc_cursor_t {}
    extern "C" {
        pub fn mongoc_cursor_is_alive(cursor: *const mongoc_cursor_t) -> u8;
        pub fn mongoc_cursor_more(cursor: *mut mongoc_cursor_t) -> u8;
        pub fn mongoc_cursor_error(cursor: *mut mongoc_cursor_t, error: *mut bson_error_t) -> u8;
        pub fn mongoc_cursor_next(cursor: *mut mongoc_cursor_t, bson: *mut *const bson_t) -> u8;
        pub fn mongoc_cursor_destroy(cursor: *mut mongoc_cursor_t) -> ();
    }

    // Bulk operation
    pub enum mongoc_bulk_operation_t {}
    extern "C" {
        pub fn mongoc_bulk_operation_insert(
            bulk: *mut mongoc_bulk_operation_t,
            document: *const bson_t,
        ) -> ();
        pub fn mongoc_bulk_operation_remove(
            bulk: *mut mongoc_bulk_operation_t,
            selector: *const bson_t,
        ) -> ();
        pub fn mongoc_bulk_operation_remove_one(
            bulk: *mut mongoc_bulk_operation_t,
            selector: *const bson_t,
        ) -> ();
        pub fn mongoc_bulk_operation_replace_one(
            bulk: *mut mongoc_bulk_operation_t,
            selector: *const bson_t,
            document: *const bson_t,
            upsert: u8,
        ) -> ();
        pub fn mongoc_bulk_operation_update(
            bulk: *mut mongoc_bulk_operation_t,
            selector: *const bson_t,
            document: *const bson_t,
            upsert: u8,
        ) -> ();
        pub fn mongoc_bulk_operation_update_one(
            bulk: *mut mongoc_bulk_operation_t,
            selector: *const bson_t,
            document: *const bson_t,
            upsert: u8,
        ) -> ();
        pub fn mongoc_bulk_operation_execute(
            bulk: *mut mongoc_bulk_operation_t,
            reply: *mut bson_t,
            error: *mut bson_error_t,
        ) -> uint32_t;
        pub fn mongoc_bulk_operation_destroy(bulk: *mut mongoc_bulk_operation_t) -> ();
    }

    // Flags
    pub type mongoc_query_flags_t = ::libc::c_uint;
    pub const MONGOC_DELETE_NONE: ::libc::c_uint = 0;
    pub const MONGOC_DELETE_SINGLE_REMOVE: ::libc::c_uint = 1;
    pub const MONGOC_REMOVE_NONE: ::libc::c_uint = 0;
    pub const MONGOC_REMOVE_SINGLE_REMOVE: ::libc::c_uint = 1;
    pub const MONGOC_INSERT_NONE: ::libc::c_uint = 0;
    pub const MONGOC_INSERT_CONTINUE_ON_ERROR: ::libc::c_uint = 1;
    pub const MONGOC_QUERY_NONE: ::libc::c_uint = 0;
    pub const MONGOC_QUERY_TAILABLE_CURSOR: ::libc::c_uint = 2;
    pub const MONGOC_QUERY_SLAVE_OK: ::libc::c_uint = 4;
    pub const MONGOC_QUERY_OPLOG_REPLAY: ::libc::c_uint = 8;
    pub const MONGOC_QUERY_NO_CURSOR_TIMEOUT: ::libc::c_uint = 16;
    pub const MONGOC_QUERY_AWAIT_DATA: ::libc::c_uint = 32;
    pub const MONGOC_QUERY_EXHAUST: ::libc::c_uint = 64;
    pub const MONGOC_QUERY_PARTIAL: ::libc::c_uint = 128;
    pub const MONGOC_REPLY_NONE: ::libc::c_uint = 0;
    pub const MONGOC_REPLY_CURSOR_NOT_FOUND: ::libc::c_uint = 1;
    pub const MONGOC_REPLY_QUERY_FAILURE: ::libc::c_uint = 2;
    pub const MONGOC_REPLY_SHARD_CONFIG_STALE: ::libc::c_uint = 4;
    pub const MONGOC_REPLY_AWAIT_CAPABLE: ::libc::c_uint = 8;
    pub const MONGOC_UPDATE_NONE: ::libc::c_uint = 0;
    pub const MONGOC_UPDATE_UPSERT: ::libc::c_uint = 1;
    pub const MONGOC_UPDATE_MULTI_UPDATE: ::libc::c_uint = 2;

    // Errors
    pub const MONGOC_ERROR_CLIENT: ::libc::c_uint = 1;
    pub const MONGOC_ERROR_STREAM: ::libc::c_uint = 2;
    pub const MONGOC_ERROR_PROTOCOL: ::libc::c_uint = 3;
    pub const MONGOC_ERROR_CURSOR: ::libc::c_uint = 4;
    pub const MONGOC_ERROR_QUERY: ::libc::c_uint = 5;
    pub const MONGOC_ERROR_INSERT: ::libc::c_uint = 6;
    pub const MONGOC_ERROR_SASL: ::libc::c_uint = 7;
    pub const MONGOC_ERROR_BSON: ::libc::c_uint = 8;
    pub const MONGOC_ERROR_MATCHER: ::libc::c_uint = 9;
    pub const MONGOC_ERROR_NAMESPACE: ::libc::c_uint = 10;
    pub const MONGOC_ERROR_COMMAND: ::libc::c_uint = 11;
    pub const MONGOC_ERROR_COLLECTION: ::libc::c_uint = 12;
    pub const MONGOC_ERROR_GRIDFS: ::libc::c_uint = 13;
    pub const MONGOC_ERROR_SCRAM: ::libc::c_uint = 14;
    pub const MONGOC_ERROR_STREAM_INVALID_TYPE: ::libc::c_uint = 1;
    pub const MONGOC_ERROR_STREAM_INVALID_STATE: ::libc::c_uint = 2;
    pub const MONGOC_ERROR_STREAM_NAME_RESOLUTION: ::libc::c_uint = 3;
    pub const MONGOC_ERROR_STREAM_SOCKET: ::libc::c_uint = 4;
    pub const MONGOC_ERROR_STREAM_CONNECT: ::libc::c_uint = 5;
    pub const MONGOC_ERROR_STREAM_NOT_ESTABLISHED: ::libc::c_uint = 6;
    pub const MONGOC_ERROR_CLIENT_NOT_READY: ::libc::c_uint = 7;
    pub const MONGOC_ERROR_CLIENT_TOO_BIG: ::libc::c_uint = 8;
    pub const MONGOC_ERROR_CLIENT_TOO_SMALL: ::libc::c_uint = 9;
    pub const MONGOC_ERROR_CLIENT_GETNONCE: ::libc::c_uint = 10;
    pub const MONGOC_ERROR_CLIENT_AUTHENTICATE: ::libc::c_uint = 11;
    pub const MONGOC_ERROR_CLIENT_NO_ACCEPTABLE_PEER: ::libc::c_uint = 12;
    pub const MONGOC_ERROR_CLIENT_IN_EXHAUST: ::libc::c_uint = 13;
    pub const MONGOC_ERROR_PROTOCOL_INVALID_REPLY: ::libc::c_uint = 14;
    pub const MONGOC_ERROR_PROTOCOL_BAD_WIRE_VERSION: ::libc::c_uint = 15;
    pub const MONGOC_ERROR_CURSOR_INVALID_CURSOR: ::libc::c_uint = 16;
    pub const MONGOC_ERROR_QUERY_FAILURE: ::libc::c_uint = 17;
    pub const MONGOC_ERROR_BSON_INVALID: ::libc::c_uint = 18;
    pub const MONGOC_ERROR_MATCHER_INVALID: ::libc::c_uint = 19;
    pub const MONGOC_ERROR_NAMESPACE_INVALID: ::libc::c_uint = 20;
    pub const MONGOC_ERROR_NAMESPACE_INVALID_FILTER_TYPE: ::libc::c_uint = 21;
    pub const MONGOC_ERROR_COMMAND_INVALID_ARG: ::libc::c_uint = 22;
    pub const MONGOC_ERROR_COLLECTION_INSERT_FAILED: ::libc::c_uint = 23;
    pub const MONGOC_ERROR_COLLECTION_UPDATE_FAILED: ::libc::c_uint = 24;
    pub const MONGOC_ERROR_COLLECTION_DELETE_FAILED: ::libc::c_uint = 25;
    pub const MONGOC_ERROR_COLLECTION_DOES_NOT_EXIST: ::libc::c_uint = 26;
    pub const MONGOC_ERROR_GRIDFS_INVALID_FILENAME: ::libc::c_uint = 27;
    pub const MONGOC_ERROR_SCRAM_NOT_DONE: ::libc::c_uint = 28;
    pub const MONGOC_ERROR_SCRAM_PROTOCOL_ERROR: ::libc::c_uint = 29;
    pub const MONGOC_ERROR_QUERY_COMMAND_NOT_FOUND: ::libc::c_uint = 59;
    pub const MONGOC_ERROR_QUERY_NOT_TAILABLE: ::libc::c_uint = 13051;
    pub const MONGOC_ERROR_PROTOCOL_ERROR: ::libc::c_uint = 17;
    pub const MONGOC_ERROR_WRITE_CONCERN_ERROR: ::libc::c_uint = 64;
    pub const MONGOC_ERROR_DUPLICATE_KEY: ::libc::c_uint = 11000;
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        // Just a quick test to make seeing if everything links right easy
    }
}
