{% if type == "CharacterSet" %}
{% if default %}DEFAULT {% endif %}CHARACTER SET {{ value }}
{% elif type == "Collate" %}
{% if default %}DEFAULT {% endif %}COLLATE {{ value }}
{% elif type == "Encryption" %}
{% if default %}DEFAULT {% endif %}ENCRYPTION='{{ value }}'
{% endif %}