{% if type ==  "TinyInt" %}
TINYINT{% if m %} ({{ m }}){% endif %}{% if unsigned %} UNSIGNED{% endif %}{% if zerofill %} ZEROFILL{% endif %}
{% elif type == "SmallInt" %}
SMALLINT{% if m %} ({{ m }}){% endif %}{% if unsigned %} UNSIGNED{% endif %}{% if zerofill %} ZEROFILL{% endif %}
{% elif type == "MediumInt" %}
MEDIUMINT{% if m %} ({{ m }}){% endif %}{% if unsigned %} UNSIGNED{% endif %}{% if zerofill %} ZEROFILL{% endif %}
{% elif type == "Int" %}
INT{% if m %} ({{ m }}){% endif %}{% if unsigned %} UNSIGNED{% endif %}{% if zerofill %} ZEROFILL{% endif %}
{% elif type == "BigInt" %}
BIGINT{% if m %} ({{ m }}){% endif %}{% if unsigned %} UNSIGNED{% endif %}{% if zerofill %} ZEROFILL{% endif %}
{% elif type == "Decimal" %}
DECIMAL{% if m %} ({{ m }}{% if d %}, {{ d }}{% endif %}){% endif %}{% if unsigned %} UNSIGNED{% endif %}{% if zerofill %} ZEROFILL{% endif %}
{% elif type == "Float" %}
FLOAT{% if m %} ({{ m }}{% if d %}, {{ d }}{% endif %}){% endif %}{% if unsigned %} UNSIGNED{% endif %}{% if zerofill %} ZEROFILL{% endif %}
{% elif type == "Double" %}
DOUBLE{% if m %} ({{ m }}{% if d %}, {{ d }}{% endif %}){% endif %}{% if unsigned %} UNSIGNED{% endif %}{% if zerofill %} ZEROFILL{% endif %}
{% elif type == "Bit" %}
BIT{% if m %} ({{ m }}){% endif %}
{% elif type == "Date" %}
DATE
{% elif type == "DateTime" %}
DATETIME{% if fsp %} ({{ fsp }}){% endif %}
{% elif type == "Timestamp" %}
TIMESTAMP{% if fsp %} ({{ fsp }}){% endif %}
{% elif type == "Time" %}
TIME{% if fsp %} ({{ fsp }}){% endif %}
{% elif type == "Year" %}
YEAR{% if m %} ({{ m }}){% endif %}
{% elif type == "Char" %}
CHAR{% if m %} ({{ m }}){% endif %}{% if charset_name %} CHARACTER SET {{ charset_name }}{% endif %}{% if collation_name %} COLLATE {{ collation_name }}{% endif %}
{% elif type == "Varchar" %}
VARCHAR{% if m %} ({{ m }}){% endif %}{% if charset_name %} CHARACTER SET {{ charset_name }}{% endif %}{% if collation_name %} COLLATE {{ collation_name }}{% endif %}
{% elif type == "Binary" %}
BINARY{% if m %} ({{ m }}){% endif %}
{% elif type == "Varbinary" %}
VARBINARY ({{ m }})
{% elif type == "Blob" %}
BLOB{% if m %} ({{ m }}){% endif %}
{% elif type == "TinyBlob" %}
TINYBLOB
{% elif type == "MediumBlob" %}
MEDIUMBLOB
{% elif type == "LongBlob" %}
LONGBLOB
{% elif type == "Text" %}
TEXT{% if m %} ({{ m }}){% endif %}{% if charset_name %} CHARACTER SET {{ charset_name }}{% endif %}{% if collation_name %} COLLATE {{ collation_name }}{% endif %}
{% elif type == "TinyText" %}
TINYTEXT{% if charset_name %} CHARACTER SET {{ charset_name }}{% endif %}{% if collation_name %} COLLATE {{ collation_name }}{% endif %}
{% elif type == "MediumText" %}
MEDIUMTEXT{% if charset_name %} CHARACTER SET {{ charset_name }}{% endif %}{% if collation_name %} COLLATE {{ collation_name }}{% endif %}
{% elif type == "LongText" %}
LONGTEXT{% if charset_name %} CHARACTER SET {{ charset_name }}{% endif %}{% if collation_name %} COLLATE {{ collation_name }}{% endif %}
{% elif type == "Enum" %}
ENUM ({% for value in values %}'{{ value }}'{% if not loop.last %}, {% endif %}{% endfor %}){% if charset_name %} CHARACTER SET {{ charset_name }}{% endif %}{% if collation_name %} COLLATE {{ collation_name }}{% endif %}
{% elif type == "Set" %}
SET ({% for value in values %}'{{ value }}'{% if not loop.last %}, {% endif %}{% endfor %}){% if charset_name %} CHARACTER SET {{ charset_name }}{% endif %}{% if collation_name %} COLLATE {{ collation_name }}{% endif %}
{% elif type == "Json" %}
JSON
{% endif %}