-- Your SQL goes here
CREATE TABLE urls (
  id INT(11) PRIMARY KEY AUTO_INCREMENT,
  `key` VARCHAR(60) NOT NULL,
  url VARCHAR(60) NOT NULL
)

INSERT INTO urls VALUES (NULL, 'x', 'y')