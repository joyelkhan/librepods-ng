param(
    [string]$Version = "1.0.0",
    [string]$Publisher = "CN=Rivers Engineering"
)

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$Root = Split-Path -Parent $ScriptDir
$Out = Join-Path $Root "target\msix"

if (Test-Path $Out) {
    Remove-Item $Out -Recurse -Force
}
New-Item -ItemType Directory -Path $Out -Force | Out-Null

cargo build --release --target x86_64-pc-windows-msvc
Copy-Item "$Root\target\x86_64-pc-windows-msvc\release\librepods.exe" "$Out\"

$ManifestXml = @"
<?xml version="1.0" encoding="utf-8"?>
<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10" 
         xmlns:mp="http://schemas.microsoft.com/appx/2014/phone/manifest"
         xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10">
  <Identity Name="LibrePodsNG" Publisher="$Publisher" Version="$Version.0" />
  <Properties>
    <DisplayName>LibrePods NG</DisplayName>
    <PublisherDisplayName>Rivers Engineering</PublisherDisplayName>
    <Logo>Assets\StoreLogo.png</Logo>
  </Properties>
  <Applications>
    <Application Id="App" StartPage="librepods.exe">
      <uap:VisualElements DisplayName="LibrePods NG" Square150x150Logo="Assets\Square150x150Logo.png" 
                          Square44x44Logo="Assets\Square44x44Logo.png" Description="Unlock AirPods features" 
                          BackgroundColor="#FFFFFF" />
    </Application>
  </Applications>
</Package>
"@

$ManifestXml | Out-File -FilePath "$Out\AppxManifest.xml" -Encoding UTF8

New-Item -ItemType Directory -Path "$Out\Assets" -Force | Out-Null

$MakePri = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\makepri.exe"
if (Test-Path $MakePri) {
    & $MakePri createconfig /cf "$Out\priconfig.xml" /dq en-US
    & $MakePri new /pr "$Out" /cf "$Out\priconfig.xml"
}

$MakeAppx = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\makeappx.exe"
if (Test-Path $MakeAppx) {
    & $MakeAppx pack /d "$Out" /p "$Root\LibrePods-$Version.msix"
    $Hash = (Get-FileHash "$Root\LibrePods-$Version.msix" -Algorithm SHA256).Hash
    "$Hash" | Out-File -FilePath "$Root\LibrePods-$Version.msix.sha256" -Encoding UTF8
}
