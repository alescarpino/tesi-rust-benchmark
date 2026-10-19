package com.benchmark;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.math.BigDecimal;

// @JsonProperty forza Jackson a leggere "item_id" (snake_case) dal JSON in ingresso:
// lo stesso identico payload generato da k6 deve restare valido anche per Rust/Node,
// che si aspettano snake_case di default.
public record ContractRowInput(
        @JsonProperty("item_id") int itemId,
        int quantity,
        BigDecimal price) {
}
