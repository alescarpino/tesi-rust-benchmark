package com.benchmark;

import java.math.BigDecimal;

public record BranchItemRevenue(String branchName, String itemName, BigDecimal totalRevenue, long lineCount) {
}
