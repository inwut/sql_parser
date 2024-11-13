SELECT name, SUM(sales)
FROM products
WHERE quantity >= 50 AND producer = "Producer"
GROUP BY category, price
ORDER BY sales DESC
LIMIT 20; 