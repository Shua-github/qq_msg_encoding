# 执行 cargo build
cargo build --release --target wasm32-unknown-unknown

# wasm 文件路径
$wasmPath = "target\wasm32-unknown-unknown\release\qq_msg_encoding.wasm"

# 读取 wasm 文件，转换成 Base64 字符串
$wasmBytes = [System.IO.File]::ReadAllBytes($wasmPath)
$wasmBase64 = [Convert]::ToBase64String($wasmBytes)

# 读取 html 模板（指定 UTF8 编码）
$htmlPath = "resources\index.html"
$htmlContent = Get-Content -Raw -Path $htmlPath -Encoding utf8

# 替换标记字符串为 wasm base64 内容
$htmlContent = $htmlContent -replace "{wasm_file_base64}", $wasmBase64

# 输出目录和文件
$outputDir = "output"
$outputHtmlPath = Join-Path $outputDir "index.html"
$outputWasmPath = Join-Path $outputDir "qq_msg_encoding.wasm"  
$outputPyPath = Join-Path $outputDir "main.py"  # 新增

# 确保输出目录存在
if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir | Out-Null
}

# 保存替换后的 HTML 到输出目录（指定 UTF8 编码）
Set-Content -Path $outputHtmlPath -Value $htmlContent -Encoding utf8

# 拷贝 .wasm 文件到输出目录
Copy-Item -Path $wasmPath -Destination $outputWasmPath -Force

# 拷贝 main.py 到输出目录
Copy-Item -Path "resources\main.py" -Destination $outputPyPath -Force

Write-Host "Build done. Output saved to $outputHtmlPath, $outputWasmPath and $outputPyPath"
