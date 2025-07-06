#! /bin/bash

set -e

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}Building native release...${NC}"
if cargo build --release; then
    echo -e "${GREEN}Native build done.${NC}"
else
    echo -e "${RED}Native build failed.${NC}"
    exit 1
fi

echo -e "${GREEN}Building x86_64-pc-windows-gnu release...${NC}"
if cargo build --target x86_64-pc-windows-gnu --release; then
    echo -e "${GREEN}Windows build done.${NC}"
else
    echo -e "${RED}Windows build failed.${NC}"
    exit 1
fi
