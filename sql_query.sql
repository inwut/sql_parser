SELECT name, SUM(sales)
FROM products JOIN
WHERE quantity = 50 AND producer = "Producer"
GROUP BY category, price1
ORDER BY sales DESC
LIMIT 20; 