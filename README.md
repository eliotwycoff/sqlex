# Sqlex

When you really wanna have a fast extraction from the tables in your database from your database dump... `sqlex` will help ya out.

## Quickstart

Create a sql dump from postgres or mysql and then run the following command:

```bash
sqlex extract --sql-file ./schema_dump.sql
```

This will output a json file with the schema of the database and run `sqlex`.
