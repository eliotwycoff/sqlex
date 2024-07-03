{% if name %}
CONSTRAINT `{{ name }}` FOREIGN KEY ({% for column_name in local_column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %}) REFERENCES `{{ foreign_table_name }}` ({% for column_name in foreign_column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %})
{% else %}
FOREIGN KEY ({% for column_name in local_column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %}) REFERENCES `{{ foreign_table_name }}` ({% for column_name in foreign_column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %})
{% endif %}