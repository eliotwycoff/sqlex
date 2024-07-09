{% if type == "AutoIncrement" %}
AUTO_INCREMENT={{ value }}
{% elif type == "CharacterSet" %}
{% if default %}DEFAULT {% endif %}CHARSET={{ value }}
{% elif type == "Collate" %}
{% if default %}DEFAULT {% endif %}COLLATE={{ value }}
{% elif type == "Comment" %}
COMMENT='{{ value }}'
{% elif type == "Engine" %}
ENGINE={{ value }}
{% elif type == "RowFormat" %}
ROW_FORMAT={{ value }}
{% elif type == "StatsPersistent" %}
STATS_PERSISTENT={{ value }}
{% endif %}