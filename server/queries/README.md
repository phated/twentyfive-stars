# Queries

The database queries can be refactored into this directory and then loaded into the application using `sqlx::query_file!()` or `sqlx::query_file_as!()` and they will still be completely type checked.
