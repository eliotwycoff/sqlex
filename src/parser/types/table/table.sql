--
-- Table structure for table `{{ name }}`
--

DROP TABLE IF EXISTS `{{ name }}`;
CREATE TABLE `{{ name }}` (
{% for specification in column_specifications %}
  {{ specification }}{% if not loop.last %},{% endif %}
{% endfor %}
)
{% if engine %}ENGINE={{ engine }} {% endif %}
{% if charset %}DEFAULT CHARSET={{ charset }} {% endif %}
{% if collate %}COLLATE={{ collate }} {% endif %}
{% if stats_persistent %}STATS_PERSISTENT={{ stats_persistent }} {% endif %}
{% if row_format %}ROW_FORMAT={{ row_format }} {% endif %}
{% if comment %}COMMENT={{ comment }} {% endif %};