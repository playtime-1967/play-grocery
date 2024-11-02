-- Create the schema
CREATE SCHEMA IF NOT EXISTS sales;

-- Define the Status enum type within the schema
CREATE TYPE sales.order_status AS ENUM ('Active', 'Delivered');

-- Create the Order table within the sales schema
CREATE TABLE sales.orders (
    id BIGSERIAL PRIMARY KEY,
    customer_id BIGINT NOT NULL,
    status sales.order_status NOT NULL,
    created_date TIMESTAMPTZ NOT NULL,
    delivery_date DATE
);

-- Create the OrderDetail table within the sales schema
CREATE TABLE sales.order_details (
    id BIGSERIAL PRIMARY KEY,
    order_id BIGINT NOT NULL REFERENCES sales.orders(id) ON DELETE CASCADE,
    product_id INT NOT NULL,
    quantity SMALLINT NOT NULL
);

-- Indexes for foreign key and frequent queries within the sales schema
CREATE INDEX idx_orders_customer ON sales.orders (customer_id);
CREATE INDEX idx_order_details_orders ON sales.order_details (order_id);
