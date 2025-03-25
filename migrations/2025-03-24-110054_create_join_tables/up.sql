-- Your SQL goes here
CREATE TABLE PropertyTypesJoin (
    property_id INT REFERENCES Properties(id),
    type_id INT REFERENCES PropertyTypes(id),
    PRIMARY KEY (property_id, type_id)
);

CREATE TABLE PropertyCategoriesJoin (
    property_id INT REFERENCES Properties(id),
    category_id INT REFERENCES PropertyCategories(id),
    PRIMARY KEY (property_id, category_id)
);
