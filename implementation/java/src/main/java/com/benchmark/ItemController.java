package com.benchmark;

import org.springframework.http.ResponseEntity;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
public class ItemController {

    private final JdbcTemplate jdbcTemplate;

    // qui si riceve il pool  che è stato creatto da spring
    public ItemController(JdbcTemplate jdbcTemplate) {
        this.jdbcTemplate = jdbcTemplate;
    }

    @GetMapping("/items/{id}")
    public ResponseEntity<Item> getItem(@PathVariable int id) {
        List<Item> items = jdbcTemplate.query(
                "SELECT id, name, price FROM items WHERE id = ?",
                (rs, rowNum) -> new Item(rs.getInt("id"), rs.getString("name"), rs.getBigDecimal("price")),
                id
        );
        if (items.isEmpty()) {
            return ResponseEntity.notFound().build();
        }
        return ResponseEntity.ok(items.get(0));
    }

    @PostMapping("/items")
    public ResponseEntity<Item> createItem(@RequestBody ItemPayload payload) {
        List<Item> items = jdbcTemplate.query(
                "INSERT INTO items (name, price) VALUES (?, ?) RETURNING id, name, price",
                (rs, rowNum) -> new Item(rs.getInt("id"), rs.getString("name"), rs.getBigDecimal("price")),
                payload.name(), payload.price()
        );
        return ResponseEntity.ok(items.get(0));
    }

    @PutMapping("/items/{id}")
    public ResponseEntity<Void> updateItem(@PathVariable int id, @RequestBody ItemPayload payload) {
        jdbcTemplate.update(
                "UPDATE items SET name = ?, price = ? WHERE id = ?",
                payload.name(), payload.price(), id
        );
        return ResponseEntity.ok().build();
    }

    @DeleteMapping("/items/{id}")
    public ResponseEntity<Void> deleteItem(@PathVariable int id) {
        jdbcTemplate.update("DELETE FROM items WHERE id = ?", id);
        return ResponseEntity.noContent().build();
    }

    @GetMapping("/reports/revenue-by-branch")
    public ResponseEntity<List<BranchItemRevenue>> revenueByBranch() {
        List<BranchItemRevenue> rows = jdbcTemplate.query(
                """
                SELECT
                    b.name AS branch_name,
                    i.name AS item_name,
                    SUM(cr.quantity * cr.price) AS total_revenue,
                    COUNT(*) AS line_count
                FROM contract_rows cr
                JOIN contracts c ON cr.contract_id = c.id
                JOIN branches b ON c.branch_id = b.id
                JOIN items i ON cr.item_id = i.id
                WHERE c.status IN ('approved', 'ongoing')
                  AND c.start_date BETWEEN '2025-01-01' AND '2025-12-31'
                GROUP BY b.name, i.name
                ORDER BY total_revenue DESC
                LIMIT 50
                """,
                (rs, rowNum) -> new BranchItemRevenue(
                        rs.getString("branch_name"),
                        rs.getString("item_name"),
                        rs.getBigDecimal("total_revenue"),
                        rs.getLong("line_count")
                )
        );
        return ResponseEntity.ok(rows);
    }
}
