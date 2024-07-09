{% if name %}
CONSTRAINT `{{ name }}` FOREIGN KEY ({% for column_name in local_column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %}) REFERENCES `{{ foreign_table_name }}` ({% for column_name in foreign_column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %}){% if on_update %} ON UPDATE {{ on_update }}{% endif %}
{% else %}
FOREIGN KEY ({% for column_name in local_column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %}) REFERENCES `{{ foreign_table_name }}` ({% for column_name in foreign_column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %}){% if on_update %} ON UPDATE {{ on_update }}{% endif %}
{% endif %}