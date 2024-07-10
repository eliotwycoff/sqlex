CREATE TABLE{% if if_not_exists%} IF NOT EXISTS{% endif %} `{{ name }}` (
{% for table_spec in table_specs %}  {{ table_spec }}{% if not loop.last %},{% endif %}
{% endfor %}){% for table_option in table_options %} {{ table_option }}{% endfor %};