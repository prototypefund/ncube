SELECT id,
       query,
       title,
       slug,
       created_at,
       updated_at
  FROM segment
 WHERE slug = ?1;
