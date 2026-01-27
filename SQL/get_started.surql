// create a record in the `category` table
// initialize with a `name` field and a `created_at` field
CREATE category SET
    name = 'Technology',
    created_at = time::now();

// set the id
CREATE person:john SET 
    name.first = 'John',
    name.last = 'Adams',
    name.full = string::join(' ', name.first, name.last),
    age = 29,
    admin = true,
    signup_at = time::now();

// populate the `category` field of the `article` record with the id of the `Technology` category.
CREATE article SET
    created_at = time::now(),
    author = person:john,
    title = 'Lorem ipsum dolor',
    text = 'Donec eleifend, nunc vitae commodo accumsan, mauris est fringilla.'
    category = (SELECT VALUE id FROM ONLY category WHERE = 'Technology' LIMIT 1);

-- Select all records from a table
SELECT * FROM article;

-- Select records from multiple tables
SELECT * FROM category, person;

-- Selecting specific records
SELECT * FROM person:john;

// - Filtering: We use the WHERE clause to only include records where the author’s age is less than 30.
// - Fetching: We can use the FETCH clause to replace record ids with their actual record values.
// - Specific fields: We only want to retrieve the title and author fields from the article table.
// - Record links: We use author.name.full to directly access the full name of the author.
SELECT title, category, author.name.full AS author_name FROM article WHERE author.age < 30 FETCH category;

UPDATE person:john SET 
    name.first = 'David',
    name.full = string::join(' ', name.first,name.last);

UPDATE person MERGE {
    age: 30,
    admin: false,
};

// return the records before they are deleted
DELETE article WHERE author.name.first = 'David' RETURN BEFORE;
