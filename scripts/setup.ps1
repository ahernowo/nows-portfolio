<#
  One-time setup for Windows 11. Installs what the project needs, builds the
  factory and newpost tools, and prepares the factory board.

  Run from the repository folder in PowerShell:

      powershell -ExecutionPolicy Bypass -File scripts\setup.ps1

  Safe to run again: each step skips what is already installed.
  Options:
      -SkipBrowser   don't download Chromium for screenshots (about 150 MB)
#>
param(
  [switch]$SkipBrowser
)

$ErrorActionPreference = "Stop"
$ZolaVersion = "0.23.6"
$Repo = Split-Path -Parent $PSScriptRoot
Set-Location $Repo

function Step($text) { Write-Host "`n== $text" -ForegroundColor Cyan }
function Ok($text) { Write-Host "   ok: $text" -ForegroundColor Green }
function Note($text) { Write-Host "   $text" -ForegroundColor Yellow }
function Has($command) { return [bool](Get-Command $command -ErrorAction SilentlyContinue) }

function Refresh-Path {
  $machine = [Environment]::GetEnvironmentVariable("Path", "Machine")
  $user = [Environment]::GetEnvironmentVariable("Path", "User")
  $env:Path = "$machine;$user;$env:USERPROFILE\.cargo\bin;$env:USERPROFILE\.local\bin"
}

function Winget-Install($id, $name) {
  if (-not (Has "winget")) {
    throw "winget is missing. Install 'App Installer' from the Microsoft Store, then run this script again."
  }
  Note "installing $name with winget..."
  winget install --id $id --exact --silent --accept-source-agreements --accept-package-agreements
  Refresh-Path
}

Refresh-Path

Step "Git"
if (Has "git") { Ok (git --version) } else { Winget-Install "Git.Git" "Git"; Ok (git --version) }

Step "C++ build tools (Rust needs the Microsoft linker)"
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$hasVcTools = $false
if (Test-Path $vswhere) {
  $found = & $vswhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
  $hasVcTools = [bool]$found
}
if ($hasVcTools) {
  Ok "Visual Studio C++ build tools found"
} else {
  Note "installing Visual Studio Build Tools with the C++ workload (a large download; a window may appear)..."
  winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --accept-source-agreements --accept-package-agreements --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
  Ok "build tools installed"
}

Step "Rust"
if (-not (Has "rustup")) { Winget-Install "Rustlang.Rustup" "Rust (rustup)" }
rustup default stable | Out-Null
rustup component add rustfmt clippy | Out-Null
rustup target add wasm32-unknown-unknown | Out-Null
if ($LASTEXITCODE -ne 0) { Note "could not add the wasm32 target; run 'rustup target add wasm32-unknown-unknown' later to build workers locally" }
Ok (rustc --version)

Step "Zola $ZolaVersion"
$zolaOk = (Has "zola") -and ((zola --version) -match [regex]::Escape($ZolaVersion))
if ($zolaOk) {
  Ok (zola --version)
} else {
  $bin = "$env:USERPROFILE\.local\bin"
  New-Item -ItemType Directory -Force -Path $bin | Out-Null
  $zip = Join-Path $env:TEMP "zola-$ZolaVersion.zip"
  $url = "https://github.com/getzola/zola/releases/download/v$ZolaVersion/zola-v$ZolaVersion-x86_64-pc-windows-msvc.zip"
  Note "downloading $url"
  Invoke-WebRequest -Uri $url -OutFile $zip
  Expand-Archive -Path $zip -DestinationPath $bin -Force
  Remove-Item $zip
  $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
  if ($userPath -notlike "*$bin*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$bin", "User")
  }
  Refresh-Path
  Ok (zola --version)
}

Step "Node.js (for wrangler and screenshots)"
if (Has "node") { Ok "node $(node --version)" } else { Winget-Install "OpenJS.NodeJS.LTS" "Node.js LTS"; Ok "node $(node --version)" }

Step "Project tools: factory and newpost"
cargo install --path tools/factory --locked --quiet
cargo install --path tools/newpost --locked --quiet
Ok (factory --version)
Ok (newpost --version)

Step "Factory board"
factory init
Ok "factory status:"
factory status

Step "Screenshot tool"
Push-Location tools/shots
npm install --no-audit --no-fund
if (-not $SkipBrowser) { npx playwright install chromium }
Pop-Location
Ok "tools/shots ready"

Step "First build"
zola --root site build
cargo test --workspace --quiet

Write-Host "`nAll set. Next: docs/setup.md, step 3 (preview the site)." -ForegroundColor Green
Write-Host "Open a NEW terminal so the updated PATH applies everywhere." -ForegroundColor Yellow
