-- Add up migration script here
CREATE TABLE IF NOT EXISTS material_group (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    sub_group_name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS material (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    price INT NOT NULL,
    description TEXT NOT NULL,
    quantity INT NOT NULL,
    mfg_date DATE NOT NULL,
    exp_date DATE NOT NULL,
    supplier_id uuid REFERENCES supplier(id),
    group_id uuid REFERENCES material_group(id),
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);
