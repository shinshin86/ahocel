#!/bin/bash
cd rust-wasm/excel
wasm-pack build --target web
cp pkg/excel_bg.wasm ../../public/
cp pkg/excel.js ../../public/
cd ../../