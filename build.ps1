$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$PSDefaultParameterValues['*:Encoding'] = 'utf8'

$repoRoot = [System.IO.Path]::GetFullPath($PSScriptRoot)
$tauriRoot = Join-Path $repoRoot 'src-tauri'
$manifestPath = Join-Path $tauriRoot 'Cargo.toml'
$configPath = Join-Path $tauriRoot 'tauri.conf.json'
$buildRoot = Join-Path $repoRoot 'build'
$releaseRoot = Join-Path $repoRoot 'release'

function Assert-DirectChild([string]$Path, [string]$ExpectedName) {
    $fullPath = [System.IO.Path]::GetFullPath($Path)
    $expectedPath = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $ExpectedName))
    if (-not $fullPath.Equals($expectedPath, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing unexpected output path: $fullPath"
    }
    return $fullPath
}

$buildRoot = Assert-DirectChild $buildRoot 'build'
$releaseRoot = Assert-DirectChild $releaseRoot 'release'

$config = Get-Content -Raw -Encoding UTF8 -LiteralPath $configPath | ConvertFrom-Json
$productName = [string]$config.productName
$version = [string]$config.version
if ([string]::IsNullOrWhiteSpace($productName) -or [string]::IsNullOrWhiteSpace($version)) {
    throw 'tauri.conf.json must define non-empty productName and version'
}

$cargoExe = (Get-Command cargo).Source
$metadataArgs = @(
    'metadata'
    '--manifest-path'
    $manifestPath
    '--no-deps'
    '--format-version'
    '1'
)
$metadataText = (& $cargoExe @metadataArgs | Out-String)
if ($LASTEXITCODE -ne 0) {
    throw "cargo metadata failed with exit code $LASTEXITCODE"
}
$metadata = $metadataText | ConvertFrom-Json
$manifestFullPath = [System.IO.Path]::GetFullPath($manifestPath)
$package = @($metadata.packages | Where-Object {
    [System.IO.Path]::GetFullPath([string]$_.manifest_path).Equals(
        $manifestFullPath,
        [System.StringComparison]::OrdinalIgnoreCase
    )
})
if ($package.Count -ne 1) {
    throw "Expected exactly one Cargo package for $manifestPath, found $($package.Count)"
}
$binaryTargets = @($package[0].targets | Where-Object { @($_.kind) -contains 'bin' })
if ($binaryTargets.Count -ne 1) {
    throw "Expected exactly one Cargo binary target, found $($binaryTargets.Count)"
}
$binaryName = [string]$binaryTargets[0].name
$sourceExe = Join-Path $tauriRoot "target\release\$binaryName.exe"
$releaseBase = "$productName-$version-win32-x64"
$unpackedDir = Join-Path $buildRoot 'win-unpacked'
$releaseDir = Join-Path $releaseRoot $releaseBase
$zipPath = Join-Path $releaseRoot "$releaseBase.zip"

foreach ($outputPath in @($buildRoot, $releaseRoot)) {
    if (Test-Path -LiteralPath $outputPath) {
        Write-Host "[build] removing stale $outputPath"
        Remove-Item -LiteralPath $outputPath -Recurse -Force
    }
}

Write-Host '[build] compiling Tauri release executable (no installers) ...'
$pnpmExe = (Get-Command pnpm).Source
$buildArgs = @('tauri', 'build', '--no-bundle')
& $pnpmExe @buildArgs
if ($LASTEXITCODE -ne 0) {
    throw "pnpm tauri build --no-bundle failed with exit code $LASTEXITCODE"
}
if (-not (Test-Path -LiteralPath $sourceExe -PathType Leaf)) {
    throw "Expected release executable was not produced: $sourceExe"
}

foreach ($directory in @(
    $unpackedDir,
    (Join-Path $unpackedDir 'data'),
    $releaseDir,
    (Join-Path $releaseDir 'data')
)) {
    New-Item -ItemType Directory -Path $directory -Force | Out-Null
}

$unpackedExe = Join-Path $unpackedDir "$binaryName.exe"
$releaseExe = Join-Path $releaseDir "$binaryName.exe"
Copy-Item -LiteralPath $sourceExe -Destination $unpackedExe
Copy-Item -LiteralPath $sourceExe -Destination $releaseExe

Write-Host "[release] compressing $releaseBase.zip ..."
Compress-Archive -LiteralPath $releaseDir -DestinationPath $zipPath -CompressionLevel Optimal

foreach ($artifact in @($unpackedExe, $releaseExe, $zipPath)) {
    if (-not (Test-Path -LiteralPath $artifact -PathType Leaf)) {
        throw "Expected artifact is missing: $artifact"
    }
    if ((Get-Item -LiteralPath $artifact).Length -le 0) {
        throw "Expected artifact is empty: $artifact"
    }
}

Write-Host '[build] complete'
Write-Host "[build] unpacked: $unpackedDir"
Write-Host "[build] release:  $releaseDir"
Write-Host "[build] zip:      $zipPath"
