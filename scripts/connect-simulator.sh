#!/usr/bin/env bash
# Script to connect to remote macOS simulator over SSH
# Establishes port forwarding for XCTestDaemon and related services

set -euo pipefail

# Configuration
MAC_HOST="${MAC_HOST:?MAC_HOST environment variable not set}"
MAC_USER="${MAC_USER:?MAC_USER environment variable not set}"
SIMULATOR_PORT="${SIMULATOR_PORT:-5037}"
TESTDAEMON_PORT="${TESTDAEMON_PORT:-5038}"
XCODE_PORT="${XCODE_PORT:-5039}"
LOCAL_PORT="${LOCAL_PORT:-5037}"

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}iOS Simulator Remote Connection Setup${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Remote Host: $MAC_HOST"
echo "Remote User: $MAC_USER"
echo "Local Port: $LOCAL_PORT"
echo ""

# Test SSH connection
echo -e "${YELLOW}Testing SSH connection...${NC}"
if ssh -o ConnectTimeout=5 "${MAC_USER}@${MAC_HOST}" "echo OK" > /dev/null 2>&1; then
    echo -e "${GREEN}✓ SSH connection successful${NC}"
else
    echo -e "${RED}✗ Failed to connect to ${MAC_USER}@${MAC_HOST}${NC}"
    echo "Please ensure:"
    echo "  1. macOS host is reachable"
    echo "  2. SSH is enabled on macOS (System Preferences > Sharing > Remote Login)"
    echo "  3. You have SSH keys configured or password available"
    exit 1
fi

# Establish port forwarding
echo ""
echo -e "${YELLOW}Establishing port forwarding...${NC}"

# Kill any existing port forwards
pkill -f "ssh.*${MAC_HOST}" || true
sleep 1

# Create SSH tunnel with port forwarding
ssh -N \
    -L "${LOCAL_PORT}:localhost:${SIMULATOR_PORT}" \
    -L "$((LOCAL_PORT+1)):localhost:${TESTDAEMON_PORT}" \
    -L "$((LOCAL_PORT+2)):localhost:${XCODE_PORT}" \
    "${MAC_USER}@${MAC_HOST}" &

SSH_PID=$!
echo "SSH tunnel PID: $SSH_PID"

# Wait for tunnel to establish
sleep 2

# Verify connection
echo -e "${YELLOW}Verifying port availability...${NC}"
if nc -z localhost "$LOCAL_PORT" 2>/dev/null; then
    echo -e "${GREEN}✓ Port forwarding established${NC}"
else
    echo -e "${RED}✗ Port forwarding failed${NC}"
    kill $SSH_PID 2>/dev/null || true
    exit 1
fi

echo ""
echo -e "${GREEN}Connection established!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Simulator accessible at: localhost:${LOCAL_PORT}"
echo "XCTestDaemon at: localhost:$((LOCAL_PORT+1))"
echo ""
echo "To disconnect, press Ctrl+C"
echo ""

# Keep tunnel alive
wait $SSH_PID
