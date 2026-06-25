# SDKWork Audio SDK Generation Script
# Generates TypeScript SDK from OpenAPI specifications

param(
    [string]$Language = "typescript",
    [string]$Surface = "all"
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir
$SdksDir = Join-Path $RootDir "sdks"

# SDK Generator path
$SdkGeneratorPath = Join-Path $RootDir "..\sdkwork-sdk-generator\bin\sdkgen.js"

if (-not (Test-Path $SdkGeneratorPath)) {
    Write-Error "SDK generator not found at: $SdkGeneratorPath"
    exit 1
}

function Generate-AppSDK {
    param([string]$Lang)

    Write-Host "Generating App SDK for language: $Lang"

    $InputSpec = Join-Path $SdksDir "sdkwork-audio-app-sdk\openapi\sdkwork-audio-app-api.openapi.yaml"
    $OutputDir = Join-Path $SdksDir "sdkwork-audio-app-sdk\sdkwork-audio-app-sdk-$Lang\generated\server-openapi"

    if (-not (Test-Path $InputSpec)) {
        Write-Error "Input spec not found: $InputSpec"
        exit 1
    }

    # Create output directory
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

    # Run SDK generator
    node $SdkGeneratorPath generate `
        --input-spec $InputSpec `
        --output-dir $OutputDir `
        --language $Lang `
        --sdk-name "sdkwork-audio-app-sdk" `
        --package-name "@sdkwork/audio-app-sdk" `
        --standard-profile sdkwork-v3

    if ($LASTEXITCODE -ne 0) {
        Write-Error "SDK generation failed for App SDK ($Lang)"
        exit 1
    }

    Write-Host "App SDK generated successfully at: $OutputDir"
}

function Generate-BackendSDK {
    param([string]$Lang)

    Write-Host "Generating Backend SDK for language: $Lang"

    $InputSpec = Join-Path $SdksDir "sdkwork-audio-backend-sdk\openapi\sdkwork-audio-backend-api.openapi.yaml"
    $OutputDir = Join-Path $SdksDir "sdkwork-audio-backend-sdk\sdkwork-audio-backend-sdk-$Lang\generated\server-openapi"

    if (-not (Test-Path $InputSpec)) {
        Write-Error "Input spec not found: $InputSpec"
        exit 1
    }

    # Create output directory
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

    # Run SDK generator
    node $SdkGeneratorPath generate `
        --input-spec $InputSpec `
        --output-dir $OutputDir `
        --language $Lang `
        --sdk-name "sdkwork-audio-backend-sdk" `
        --package-name "@sdkwork/audio-backend-sdk" `
        --standard-profile sdkwork-v3

    if ($LASTEXITCODE -ne 0) {
        Write-Error "SDK generation failed for Backend SDK ($Lang)"
        exit 1
    }

    Write-Host "Backend SDK generated successfully at: $OutputDir"
}

# Main execution
Write-Host "Starting SDK generation..."
Write-Host "Language: $Language"
Write-Host "Surface: $Surface"

if ($Surface -eq "all" -or $Surface -eq "app") {
    Generate-AppSDK -Lang $Language
}

if ($Surface -eq "all" -or $Surface -eq "backend") {
    Generate-BackendSDK -Lang $Language
}

Write-Host "SDK generation completed successfully!"
