#!/bin/bash
# test_maid.sh - Test script for the Maid CLI tool

set -e

echo "🧪 Testing Maid CLI tool..."

# Create test directory
TEST_DIR="/tmp/maid-test-$(date +%s)"
mkdir -p "$TEST_DIR"

echo "📁 Created test directory: $TEST_DIR"

# Create test files
echo "📝 Creating test files..."

# Create MD files with different naming patterns
cat > "$TEST_DIR/DOCUMENTATION_REFACTORING_SUMMARY.md" << EOF
# Documentation Refactoring Summary

This document summarizes the refactoring of our documentation system.

## Changes Made
- Reorganized the docs folder
- Added better navigation
- Improved search functionality
EOF

cat > "$TEST_DIR/USER_GUIDE_V1.md" << EOF
# User Guide

This guide explains how to use our product.

## Getting Started
1. Install the software
2. Configure your settings
3. Enjoy!
EOF

cat > "$TEST_DIR/IMPLEMENTATION_RUBRIC.md" << EOF
# Implementation Rubric

## Scoring Criteria
- Code quality: 25%
- Performance: 25%
- Documentation: 25%
- Testing: 25%
EOF

cat > "$TEST_DIR/STATUS_REPORT_Q2.md" << EOF
# Q2 Status Report

Project is on track and meeting all milestones.

## Key Metrics
- Completed 85% of planned features
- Fixed 47 bugs
- Added 12,000 lines of code
EOF

# Create SH files
cat > "$TEST_DIR/setup-environment.sh" << EOF
#!/bin/bash
# Setup script for the development environment

echo "Setting up development environment..."
mkdir -p ./build
npm install
EOF
chmod +x "$TEST_DIR/setup-environment.sh"

cat > "$TEST_DIR/run_tests.sh" << EOF
#!/bin/bash
# Run test suite

echo "Running tests..."
npm test
EOF
chmod +x "$TEST_DIR/run_tests.sh"

echo "🧪 Running tests..."

# Test 1: Basic clean
echo "Test 1: Basic clean"
cd "$TEST_DIR" && maid clean --verbose

# Test 2: Clean with restructure
echo "Test 2: Clean with restructure"
cd "$TEST_DIR" && maid clean --restructure --verbose

# Test 3: Dry run
echo "Test 3: Dry run"
cd "$TEST_DIR" && maid clean --dry-run --verbose

# Test 4: Keep command
echo "Test 4: Keep command"
cd "$TEST_DIR" && maid keep --verbose

echo "✅ All tests completed successfully!"
echo "🧹 Test directory: $TEST_DIR"
echo "📝 You can inspect the results manually or delete with: rm -rf $TEST_DIR"
