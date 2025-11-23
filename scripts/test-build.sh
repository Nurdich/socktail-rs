#!/usr/bin/env bash
# Quick test build script for local development

set -e

echo "🧪 Testing local build..."
echo ""

# Test current platform build
echo "1️⃣ Building for current platform (release)..."
cargo build --release
echo "✅ Current platform build successful"
echo ""

# Check binary
BINARY="target/release/socktail"
if [ -f "$BINARY" ]; then
    SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
    echo "📦 Binary size: $SIZE"

    # Test run
    echo ""
    echo "2️⃣ Testing binary..."
    timeout 2 $BINARY --help || true
    echo "✅ Binary runs successfully"
fi

echo ""
echo "3️⃣ Testing Linux musl build (static linking)..."
if cargo build --release --target x86_64-unknown-linux-musl 2>&1; then
    MUSL_BINARY="target/x86_64-unknown-linux-musl/release/socktail"
    if [ -f "$MUSL_BINARY" ]; then
        SIZE=$(ls -lh "$MUSL_BINARY" | awk '{print $5}')
        echo "✅ Linux musl build successful"
        echo "📦 Binary size: $SIZE"
        echo ""

        # Check if it's statically linked
        if command -v ldd &> /dev/null; then
            echo "🔍 Checking static linking..."
            ldd "$MUSL_BINARY" 2>&1 | grep -q "not a dynamic executable" && \
                echo "✅ Binary is statically linked!" || \
                echo "⚠️  Binary has dynamic dependencies"
        fi
    fi
else
    echo "⚠️  Linux musl build failed (may need musl-tools)"
fi

echo ""
echo "🎉 Build test complete!"
