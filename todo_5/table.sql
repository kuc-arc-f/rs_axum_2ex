CREATE TABLE IF NOT EXISTS todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT,
    content TEXT,
    is_public BOOLEAN,
    food_orange BOOLEAN,
    food_apple BOOLEAN,
    food_banana BOOLEAN,
    pub_date TEXT,
    qty1 INTEGER,
    qty2 INTEGER,
    qty3 INTEGER,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
