DEFINE FIELD lastName ON planet TYPE string;
UPDATE planet SET lastName = firstName;
REMOVE FIELD firstName ON TABLE planet;
DEFINE EVENT event2 ON animal WHEN species = 'Homo Sapien' AND speed < 10 THEN (SELECT * FROM eats);
DEFINE EVENT event1 ON animal WHEN species = 'Homo Erectus' AND speed > 545 THEN (SELECT * FROM crop);
DEFINE INDEX species_speed_idx ON animal FIELDS species, speed UNIQUE;
DEFINE FIELD characteristics ON animal TYPE array;
UPDATE animal SET characteristics = attributes;
REMOVE FIELD attributes ON TABLE animal;
DEFINE FIELD speed ON animal TYPE int;
REMOVE FIELD perre ON TABLE animal;
DEFINE FIELD color ON crop TYPE string;
REMOVE FIELD colour ON TABLE crop;