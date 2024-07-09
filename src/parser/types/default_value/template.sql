{% if type == "Null" %}
NULL
{% elif type == "CurrentTimestamp" %}
CURRENT_TIMESTAMP{% if value %} ({{ value }}){% endif %}
{% elif type == "Text" %}
'{{ value }}'
{% elif type == "Number" %}
{{ value }}
{% endif %}