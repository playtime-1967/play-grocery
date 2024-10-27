
CREATE SCHEMA IF NOT EXISTS product;

CREATE TABLE product.categories (
    id SMALLSERIAL PRIMARY KEY,  -- Auto-incrementing ID for u16
    name VARCHAR(255) NOT NULL
);

-- Create the Product table
CREATE TABLE product.products (
    id SERIAL PRIMARY KEY,  -- Auto-incrementing ID for u32
    category_id SMALLINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    price FLOAT4 NOT NULL,  -- f32

    -- Foreign key constraint to ensure category_id corresponds to an existing Category
    FOREIGN KEY (category_id) REFERENCES product.categories(id) ON DELETE CASCADE
);