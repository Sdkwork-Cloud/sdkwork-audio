# SDKWork Audio Verification Script
# Runs all verification checks

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir

Write-Host "Starting verification..."
Write-Host "Root directory: $RootDir"

# Change to root directory
Push-Location $RootDir

try {
    # 1. TypeScript type checking
    Write-Host "`n=== TypeScript Type Checking ==="
    pnpm typecheck
    if ($LASTEXITCODE -ne 0) {
        Write-Error "TypeScript type checking failed"
        exit 1
    }
    Write-Host "TypeScript type checking passed"

    # 2. TypeScript tests
    Write-Host "`n=== TypeScript Tests ==="
    pnpm test -- --run
    if ($LASTEXITCODE -ne 0) {
        Write-Error "TypeScript tests failed"
        exit 1
    }
    Write-Host "TypeScript tests passed"

    # 3. Rust tests
    Write-Host "`n=== Rust Tests ==="
    pnpm test:rust
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Rust tests failed"
        exit 1
    }
    Write-Host "Rust tests passed"

    # 4. API contract validation
    Write-Host "`n=== API Contract Validation ==="
    $AppApiPath = Join-Path $RootDir "apis\app-api\audio\openapi.yaml"
    $BackendApiPath = Join-Path $RootDir "apis\backend-api\audio\openapi.yaml"

    if (-not (Test-Path $AppApiPath)) {
        Write-Error "App API spec not found: $AppApiPath"
        exit 1
    }

    if (-not (Test-Path $BackendApiPath)) {
        Write-Error "Backend API spec not found: $BackendApiPath"
        exit 1
    }

    Write-Host "API contract validation passed"

    # 5. SDK assembly validation
    Write-Host "`n=== SDK Assembly Validation ==="
    $AppAssemblyPath = Join-Path $RootDir "sdks\sdkwork-audio-app-sdk\.sdkwork-assembly.json"
    $BackendAssemblyPath = Join-Path $RootDir "sdks\sdkwork-audio-backend-sdk\.sdkwork-assembly.json"

    if (-not (Test-Path $AppAssemblyPath)) {
        Write-Error "App SDK assembly not found: $AppAssemblyPath"
        exit 1
    }

    if (-not (Test-Path $BackendAssemblyPath)) {
        Write-Error "Backend SDK assembly not found: $BackendAssemblyPath"
        exit 1
    }

    # Validate JSON
    Get-Content $AppAssemblyPath | ConvertFrom-Json | Out-Null
    Get-Content $BackendAssemblyPath | ConvertFrom-Json | Out-Null

    Write-Host "SDK assembly validation passed"

    # 6. Database migration validation
    Write-Host "`n=== Database Migration Validation ==="
    $MigrationsPath = Join-Path $RootDir "crates\sdkwork-audio-generation-repository-sqlx\migrations"

    if (-not (Test-Path $MigrationsPath)) {
        Write-Error "Migrations directory not found: $MigrationsPath"
        exit 1
    }

    $MigrationFiles = Get-ChildItem $MigrationsPath -Filter "*.sql"
    if ($MigrationFiles.Count -eq 0) {
        Write-Error "No migration files found"
        exit 1
    }

    Write-Host "Database migration validation passed ($($MigrationFiles.Count) migrations found)"

    # 7. Documentation validation
    Write-Host "`n=== Documentation Validation ==="
    $DocsPath = Join-Path $RootDir "docs"

    $RequiredDocs = @(
        "AUDIO_APP_DESIGN_REPORT.md",
        "DATABASE_DESIGN.md",
        "API_DESIGN.md",
        "SDK_DESIGN.md",
        "ALIGNMENT_REPORT.md"
    )

    foreach ($Doc in $RequiredDocs) {
        $DocPath = Join-Path $DocsPath $Doc
        if (-not (Test-Path $DocPath)) {
            Write-Error "Required document not found: $Doc"
            exit 1
        }
    }

    Write-Host "Documentation validation passed"

    Write-Host "`n=== All Verification Checks Passed ==="
}
finally {
    Pop-Location
}
