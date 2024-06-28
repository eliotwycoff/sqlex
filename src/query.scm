(
  create_table_statement
    (identifier) @table_name
    (table_parameters
      (table_column
        (identifier) @column_name
        (type (identifier) @column_type)
      )+
    )+
)