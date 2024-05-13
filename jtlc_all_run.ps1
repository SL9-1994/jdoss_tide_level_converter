# Windows PowerShell script to run jtlc.exe on all subdirectories of a given directory.
# Usage: .\jtlc_all_run.ps1 -Path "C:\path\to\directory"

# jtlc.exeを指定されたディレクトリの、全てのサブディレクトリで実行するWindows PowerShell Scriptです。
# 実行する場所は、jtlc.exe があるディレクトリであることが必要です。
# 使い方: .\jtlc_all_run.ps1 -Path "C:\path\to\directory"

param (
    [string]$Path
)

$Subdirectories = Get-ChildItem -Directory -Path $Path

foreach ($Subdir in $Subdirectories) {
    .\jtlc.exe -d "$Path\$Subdir"
}

Write-Output "Conversion process completed."