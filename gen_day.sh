#!/bin/bash

# Setup: Put your session cookie in a file named .session in the same folder
SESSION_FILE=".session"

# Colors
GREEN="\033[0;32m"
BLUE="\033[0;34m"
RED="\033[0;31m"
YELLOW="\033[0;33m"
BOLD="\033[1m"
RESET="\033[0m"

# 1. Argument Parsing
if [ -z "$1" ]; then
    echo -e "${RED}Usage: $0 <day_number>${RESET}"
    echo "Example: $0 6"
    exit 1
fi

# Normalize Day (6 -> 06) and Integer Day (06 -> 6)
DAY_PAD=$(printf "%02d" "$1")
DAY_INT=$((10#$DAY_PAD)) # Strip leading zero for URL
DIR="./$DAY_PAD"

# 2. Get Year from Git Branch (Fall back to system date)
BRANCH_NAME=$(git branch --show-current 2>/dev/null)
# Extract 4 consecutive digits that start with 20
YEAR=$(echo "$BRANCH_NAME" | grep -oE '20[0-9]{2}' | head -n 1)

if [ -z "$YEAR" ]; then
    YEAR=$(date +%Y)
    echo -e "${YELLOW}⚠️  No year found in branch name '$BRANCH_NAME'. Defaulting to system year: $YEAR${RESET}"
else
    echo -e "${BLUE}📅 Detected Year from Git: ${BOLD}$YEAR${RESET}"
fi

echo -e "${BLUE}${BOLD}🎄 Scaffolding Day $DAY_PAD...${RESET}"
mkdir -p "$DIR"

# 3. Download Real Input (Requires Session)
if [ -f "$DIR/input.txt" ]; then
    echo -e "${GREEN}✓ Input already exists.${RESET}"
else
    if [ -f "$SESSION_FILE" ]; then
        echo -e "${BLUE}⬇️  Downloading input...${RESET}"
        COOKIE=$(cat "$SESSION_FILE" | tr -d '\n')
        # Check if cookie starts with "session=", if not, add it
        if [[ "$COOKIE" != session=* ]]; then COOKIE="session=$COOKIE"; fi
        
        HTTP_STATUS=$(curl -s -w "%{http_code}" -o "$DIR/input.txt" --cookie "$COOKIE" "https://adventofcode.com/$YEAR/day/$DAY_INT/input")
        
        if [ "$HTTP_STATUS" -ne 200 ]; then
            echo -e "${RED}💀 Download failed (HTTP $HTTP_STATUS). Check your cookie.${RESET}"
            rm "$DIR/input.txt" 
        else
            echo -e "${GREEN}✓ Input downloaded.${RESET}"
        fi
    else
        echo -e "${RED}⚠️  No .session file found. Created empty input.txt.${RESET}"
        touch "$DIR/input.txt"
    fi
fi

# 4. Scrape Sample Input (No Session Needed)
# We fetch the problem page and look for the first <pre><code> block
if [ -f "$DIR/sample.txt" ] && [ -s "$DIR/sample.txt" ]; then
    echo -e "${GREEN}✓ Sample already exists.${RESET}"
else
    echo -e "${BLUE}🕵️  Scraping sample from problem description...${RESET}"
    PAGE_HTML=$(curl -s "https://adventofcode.com/$YEAR/day/$DAY_INT")
    
    # Use Python to extract the first code block safely and unescape HTML entities
    SAMPLE_CONTENT=$(echo "$PAGE_HTML" | python3 -c "
import sys, re, html
content = sys.stdin.read()
# Find content between <pre><code> and </code></pre>
match = re.search(r'<pre><code>(.*?)</code></pre>', content, re.DOTALL)
if match:
    text = match.group(1)
    print(html.unescape(text).strip())
else:
    sys.exit(1)
")
    
    if [ $? -eq 0 ] && [ ! -z "$SAMPLE_CONTENT" ]; then
        echo "$SAMPLE_CONTENT" > "$DIR/sample.txt"
        echo -e "${GREEN}✓ Sample scraped successfully.${RESET}"
    else
        echo -e "${YELLOW}⚠️  Could not auto-detect sample. Created empty file.${RESET}"
        touch "$DIR/sample.txt"
    fi
fi

# 5. Generate Code & Update Cargo
TEMPLATE='use aoc; 
use itertools::Itertools;

const EXPECTED: usize = 0;

aoc::solution!(EXPECTED, solve);

fn solve(input: &str) -> usize {
    0
}'

for PART in 1 2; do
    FILE="$DIR/part$PART.rs"
    BIN_NAME="${DAY_INT}_${PART}"

    if [ -f "$FILE" ]; then
        echo -e "${GREEN}✓ $FILE already exists. Skipping.${RESET}"
    else
        echo "$TEMPLATE" > "$FILE"
        echo -e "${GREEN}✓ Created $FILE${RESET}"
    fi

    if grep -q "name = \"$BIN_NAME\"" Cargo.toml; then
        echo -e "${GREEN}✓ Binary $BIN_NAME already in Cargo.toml${RESET}"
    else
        cat <<EOT >> Cargo.toml

[[bin]]
name = "$BIN_NAME"
path = "$DIR/part$PART.rs"
EOT
        echo -e "${BLUE}💀 Injected binary $BIN_NAME into Cargo.toml${RESET}"
    fi
done

echo -e "${BOLD}${GREEN}🚀 Ready. Run: cargo run --bin ${DAY_PAD}_1${RESET}"