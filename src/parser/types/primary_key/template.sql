{% if name %}
CONSTRAINT `{{ name }}` PRIMARY KEY ({% for column_name in column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %})
{% else %}
PRIMARY KEY ({% for column_name in column_names %}`{{ column_name }}`{% if not loop.last %},{% endif %}{% endfor %})
{% endif %}