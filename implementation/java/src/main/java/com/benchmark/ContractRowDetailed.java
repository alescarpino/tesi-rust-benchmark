package com.benchmark;

import com.fasterxml.jackson.annotation.JsonFormat;

import java.math.BigDecimal;

public record ContractRowDetailed(
        int contractId,
        BranchRef branch,
        ItemRef item,
        int quantity,
        // forzato a stringa per restare byte-comparabile con Rust/Node, che serializzano
        // i decimali come stringa di default (vedi discussione: non è una differenza di
        // performance tra linguaggi, solo un default diverso di Jackson da controllare)
        @JsonFormat(shape = JsonFormat.Shape.STRING) BigDecimal price,
        long lineNumber) {
}
