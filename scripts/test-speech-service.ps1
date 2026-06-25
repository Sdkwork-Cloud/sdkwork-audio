# Test Speech Service Script

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir

Write-Host "Testing Speech Service..."
Write-Host "Root directory: $RootDir"

# Change to root directory
Push-Location $RootDir

try {
    # Run tests for speech service
    Write-Host "`n=== Running Speech Service Tests ==="
    cargo test -p sdkwork-audio-speech-service --verbose

    if ($LASTEXITCODE -ne 0) {
        Write-Error "Speech service tests failed"
        exit 1
    }

    Write-Host "`n=== Speech Service Tests Passed ==="
}
finally {
    Pop-Location
}
