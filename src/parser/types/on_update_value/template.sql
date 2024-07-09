{% if type == "Null" %}
NULL
{% elif type == "CurrentTimestamp" %}
CURRENT_TIMESTAMP{% if value %} ({{ value }}){% endif %}
{% endif %}