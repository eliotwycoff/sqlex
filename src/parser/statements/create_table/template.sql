CREATE TABLE `{{ name }}` (
{% for table_spec in table_specs %}  {{ table_spec }}{% if not loop.last %},{% endif %}
{% endfor %}){% for table_option in table_options %} {{ table_option }}{% endfor %};