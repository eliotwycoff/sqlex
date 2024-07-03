--
-- Current Database: {{ name }}
--

CREATE DATABASE IF NOT EXISTS {{ name }} {% for option in options %}{% if loop.first %}DEFAULT {% endif %}{{ option }} {% endfor %};

USE `{{ name }}`;

{% for table in tables %}
{{ table }}
{% endfor %}