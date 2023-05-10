#!/bin/bash

# get hightlight default css
curl -x socks5h://localhost:1080 https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/styles/default.min.css -o ./src/assets/default.min.css

# get hightlight.js
curl -x socks5h://localhost:1080 https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/highlight.min.js -o ./src/assets/highlight.min.js

# get markdown js
curl -x socks5h://localhost:1080 https://cdn.jsdelivr.net/npm/marked/marked.min.js -o ./src/assets/marked.min.js