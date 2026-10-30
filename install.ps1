$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

$repo = 'yaso09/openpixel'
$installDir = if ($env:OPENPIXEL_INSTALL_DIR) { $env:OPENPIXEL_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA 'OpenPixel\bin' }
$arch = $env:PROCESSOR_ARCHITECTURE
if ($env:PROCESSOR_ARCHITEW6432) { $arch = $env:PROCESSOR_ARCHITEW6432 }
if ($arch -notin @('AMD64', 'x64')) { throw "Unsupported architecture: $arch. OpenPixel currently provides Windows x64 releases." }

$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest"
$asset = $release.assets | Where-Object { $_.name -eq 'openpixel-windows-x86_64.zip' } | Select-Object -First 1
if (-not $asset) { throw "The latest OpenPixel release does not contain the Windows x64 package." }

$tempDir = Join-Path ([System.IO.Path]::GetTempPath()) ([guid]::NewGuid().ToString())
$archive = Join-Path $tempDir 'openpixel.zip'
New-Item -ItemType Directory -Path $tempDir | Out-Null
try {
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $archive
    Expand-Archive -LiteralPath $archive -DestinationPath $tempDir
    New-Item -ItemType Directory -Force -Path $installDir | Out-Null
    Copy-Item -LiteralPath (Join-Path $tempDir 'openpixel.exe') -Destination $installDir -Force
    Copy-Item -LiteralPath (Join-Path $tempDir 'openpixel-mcp.exe') -Destination $installDir -Force

    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $pathEntries = @($userPath -split ';' | Where-Object { $_ })
    if ($pathEntries -notcontains $installDir) {
        [Environment]::SetEnvironmentVariable('Path', (($pathEntries + $installDir) -join ';'), 'User')
    }
    $env:Path = "$installDir;$env:Path"
    Write-Output "Installed OpenPixel to $installDir"
    Write-Output 'Open a new terminal to use openpixel and openpixel-mcp.'

    if (Get-Command npx -ErrorAction SilentlyContinue) {
        & npx --yes skills add https://github.com/yaso09/openpixel/skills --skill openpixel-pixel-art --global --yes
        if ($LASTEXITCODE -ne 0) {
            Write-Warning 'OpenPixel tools are installed, but the agent skill could not be installed. Run the skill install command from the README to retry.'
        }
    }
    else {
        Write-Warning 'Node.js/npm was not found, so the agent skill was skipped. Install Node.js and run: npx skills add https://github.com/yaso09/openpixel/skills --skill openpixel-pixel-art --global --yes'
    }
}
finally {
    Remove-Item -LiteralPath $tempDir -Recurse -Force -ErrorAction SilentlyContinue
}
